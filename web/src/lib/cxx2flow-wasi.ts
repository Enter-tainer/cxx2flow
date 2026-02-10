import { WASI } from '@bjorn3/browser_wasi_shim'
import wasmUrl from '@/wasm/cxx2flow_bg.wasm?url'

type Cxx2flowExports = {
  memory: WebAssembly.Memory
  cxx2flow_alloc: (size: number) => number
  cxx2flow_dealloc: (ptr: number, size: number) => void
  cxx2flow_generate_dot: (
    contentPtr: number,
    contentLen: number,
    functionPtr: number,
    functionLen: number,
    curly: number,
  ) => number
  cxx2flow_result_ptr: () => number
  cxx2flow_result_len: () => number
  cxx2flow_error_ptr: () => number
  cxx2flow_error_len: () => number
}

let wasm: Cxx2flowExports | null = null
let initPromise: Promise<Cxx2flowExports> | null = null
let cachedMemory: Uint8Array | null = null

const textEncoder = new TextEncoder()
const textDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })

function getMemoryView() {
  if (!wasm) {
    throw new Error('cxx2flow wasm is not initialized')
  }
  if (!cachedMemory || cachedMemory.buffer !== wasm.memory.buffer) {
    cachedMemory = new Uint8Array(wasm.memory.buffer)
  }
  return cachedMemory
}

function getString(ptr: number, len: number) {
  if (len === 0) {
    return ''
  }
  return textDecoder.decode(getMemoryView().subarray(ptr, ptr + len))
}

function writeString(value: string): [number, number] {
  if (!wasm) {
    throw new Error('cxx2flow wasm is not initialized')
  }
  const bytes = textEncoder.encode(value)
  if (bytes.length === 0) {
    return [0, 0]
  }

  const ptr = wasm.cxx2flow_alloc(bytes.length)
  if (ptr === 0) {
    throw new Error('failed to allocate wasm memory')
  }

  getMemoryView().set(bytes, ptr)
  return [ptr, bytes.length]
}

export async function initWasm() {
  if (wasm) {
    return wasm
  }
  if (initPromise) {
    return initPromise
  }

  initPromise = (async () => {
    const wasi = new WASI([], [], [])
    const imports = {
      wasi_snapshot_preview1: wasi.wasiImport,
    }

    const response = await fetch(wasmUrl)
    if (!response.ok) {
      throw new Error(`failed to fetch wasm: ${response.status} ${response.statusText}`)
    }

    const bytes = await response.arrayBuffer()
    const { instance } = await WebAssembly.instantiate(bytes, imports)
    wasi.initialize(instance as unknown as { exports: { memory: WebAssembly.Memory; _initialize?: () => unknown } })

    wasm = instance.exports as unknown as Cxx2flowExports
    cachedMemory = null
    return wasm
  })()

  try {
    return await initPromise
  } finally {
    if (!wasm) {
      initPromise = null
    }
  }
}

export function generate_dot(content: string, functionName?: string, curly = false) {
  if (!wasm) {
    throw new Error('cxx2flow wasm is not initialized. Call initWasm() first.')
  }

  const normalizedContent = content ?? ''
  const normalizedFunction = functionName ?? ''

  let contentPtr = 0
  let contentLen = 0
  let functionPtr = 0
  let functionLen = 0

  try {
    ;[contentPtr, contentLen] = writeString(normalizedContent)
    ;[functionPtr, functionLen] = writeString(normalizedFunction)

    const status = wasm.cxx2flow_generate_dot(
      contentPtr,
      contentLen,
      functionPtr,
      functionLen,
      curly ? 1 : 0,
    )

    if (status === 0) {
      return getString(wasm.cxx2flow_result_ptr(), wasm.cxx2flow_result_len())
    }

    const message = getString(wasm.cxx2flow_error_ptr(), wasm.cxx2flow_error_len())
    throw new Error(message || `cxx2flow_generate_dot failed with status ${status}`)
  } finally {
    if (contentLen > 0) {
      wasm.cxx2flow_dealloc(contentPtr, contentLen)
    }
    if (functionLen > 0) {
      wasm.cxx2flow_dealloc(functionPtr, functionLen)
    }
  }
}

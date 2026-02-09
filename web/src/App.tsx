import { useEffect, useMemo, useState } from 'react'
import { AlertCircle, RotateCcw, Sparkles } from 'lucide-react'
import { Graphviz } from '@hpcc-js/wasm-graphviz'
import initWasm, { generate_dot } from 'cxx2flow'
import wasmUrl from 'cxx2flow/cxx2flow_bg.wasm?url'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'

const DEFAULT_SOURCE = `int main() {
  int x = 0;
  for (int i = 0; i < 5; i++) {
    x += i;
  }
  if (x > 3) {
    return x;
  }
  return 0;
}`

type RenderState = 'idle' | 'loading' | 'ready' | 'error'

export default function App() {
  const [sourceCode, setSourceCode] = useState(DEFAULT_SOURCE)
  const [functionName, setFunctionName] = useState('main')
  const [lineStyle, setLineStyle] = useState<'polyline' | 'curly'>('polyline')
  const [dotText, setDotText] = useState('')
  const [svgText, setSvgText] = useState('')
  const [errorText, setErrorText] = useState('')
  const [status, setStatus] = useState<RenderState>('idle')
  const [isEngineLoading, setIsEngineLoading] = useState(true)

  const [isWasmReady, setIsWasmReady] = useState(false)
  const [graphvizEngine, setGraphvizEngine] = useState<Awaited<ReturnType<typeof Graphviz.load>> | null>(null)

  const canRender = useMemo(() => Boolean(isWasmReady && graphvizEngine), [isWasmReady, graphvizEngine])

  useEffect(() => {
    let cancelled = false

    async function bootstrap() {
      try {
        setIsEngineLoading(true)

        const [wasmBinary, gv] = await Promise.all([
          fetch(wasmUrl).then(async (response) => {
            if (!response.ok) {
              throw new Error(`Failed to fetch wasm: ${response.status} ${response.statusText}`)
            }
            return response.arrayBuffer()
          }),
          Graphviz.load(),
        ])

        await initWasm(wasmBinary)

        if (cancelled) {
          return
        }

        setIsWasmReady(true)
        setGraphvizEngine(gv)
        setStatus('ready')
      } catch (error) {
        if (cancelled) {
          return
        }
        const message = error instanceof Error ? error.message : String(error)
        setStatus('error')
        setErrorText(`Initialization failed: ${message}`)
      } finally {
        if (!cancelled) {
          setIsEngineLoading(false)
        }
      }
    }

    void bootstrap()

    return () => {
      cancelled = true
    }
  }, [])

  useEffect(() => {
    if (!canRender || isEngineLoading) {
      return
    }

    const engine = graphvizEngine
    if (!engine) {
      return
    }

    if (!sourceCode.trim()) {
      setDotText('')
      setSvgText('')
      setErrorText('')
      setStatus('ready')
      return
    }

    setStatus('loading')
    const timer = window.setTimeout(() => {
      try {
        const nextDot = generate_dot(sourceCode, functionName || undefined, lineStyle === 'curly')
        const nextSvg = engine.dot(nextDot)

        setDotText(nextDot)
        setSvgText(nextSvg)
        setErrorText('')
        setStatus('ready')
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error)
        setStatus('error')
        setErrorText(message)
      }
    }, 220)

    return () => {
      window.clearTimeout(timer)
    }
  }, [canRender, sourceCode, functionName, lineStyle, graphvizEngine, isEngineLoading])

  const onReset = () => {
    setSourceCode(DEFAULT_SOURCE)
    setFunctionName('main')
    setLineStyle('polyline')
    setDotText('')
    setSvgText('')
    setErrorText('')
    setStatus(canRender ? 'ready' : 'idle')
  }

  const statusText = isEngineLoading ? 'Loading wasm' : status === 'loading' ? 'Rendering' : 'Auto'

  return (
    <main className="min-h-screen bg-background text-foreground">
      <div className="mx-auto flex w-full max-w-[1600px] flex-col gap-4 p-4 md:p-6">
        <header className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
          <div className="space-y-1">
            <h1 className="text-2xl font-semibold tracking-tight">cxx2flow Web Playground</h1>
            <p className="text-sm text-muted-foreground">Left: C/C++ source; Right: Graphviz SVG rendered in browser</p>
          </div>
        </header>

        <section className="grid grid-cols-1 gap-4 xl:grid-cols-2">
          <Card className="h-[75vh]">
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 text-base">
                <Sparkles className="h-4 w-4" />
                Source
              </CardTitle>
              <CardDescription>Paste C/C++ and preview updates automatically</CardDescription>
            </CardHeader>
            <CardContent className="flex h-[calc(100%-88px)] flex-col gap-3">
              <div className="grid grid-cols-1 gap-2 md:grid-cols-[1fr_220px_220px]">
                <Input value={functionName} onChange={(event) => setFunctionName(event.target.value)} placeholder="function name" />
                <Select value={lineStyle} onValueChange={(value) => setLineStyle(value as 'polyline' | 'curly')}>
                  <SelectTrigger>
                    <SelectValue placeholder="Line style" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="polyline">polyline</SelectItem>
                    <SelectItem value="curly">curly</SelectItem>
                  </SelectContent>
                </Select>
                <div className="flex items-center justify-end gap-2">
                  <Badge variant="outline">{statusText}</Badge>
                  <Button variant="outline" onClick={onReset}>
                    <RotateCcw className="h-4 w-4" />
                  </Button>
                </div>
              </div>

              <Textarea
                className="h-full min-h-[420px] resize-none font-mono text-xs leading-relaxed"
                value={sourceCode}
                onChange={(event) => setSourceCode(event.target.value)}
                spellCheck={false}
              />
            </CardContent>
          </Card>

          <Card className="h-[75vh]">
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 text-base">
                <Sparkles className="h-4 w-4" />
                Graphviz Preview
              </CardTitle>
              <CardDescription>{status === 'loading' || isEngineLoading ? 'Rendering...' : 'Rendered SVG from DOT output'}</CardDescription>
            </CardHeader>
            <CardContent className="h-[calc(100%-88px)]">
              {status === 'error' ? (
                <div className="flex h-full flex-col gap-3 rounded-md border border-red-500/40 bg-red-500/5 p-4 text-sm">
                  <div className="flex items-center gap-2 text-red-600">
                    <AlertCircle className="h-4 w-4" />
                    Render failed
                  </div>
                  <pre className="overflow-auto whitespace-pre-wrap text-xs text-red-700">{errorText}</pre>
                </div>
              ) : svgText ? (
                <div className="h-full overflow-auto rounded-md border bg-white p-2" dangerouslySetInnerHTML={{ __html: svgText }} />
              ) : (
                <div className="flex h-full items-center justify-center rounded-md border border-dashed text-sm text-muted-foreground">
                  Auto-rendering after edits
                </div>
              )}
            </CardContent>
          </Card>
        </section>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm">DOT Output</CardTitle>
          </CardHeader>
          <CardContent>
            <pre className="max-h-56 overflow-auto rounded-md border bg-muted/50 p-3 text-xs leading-relaxed">{dotText || 'No DOT yet'}</pre>
          </CardContent>
        </Card>
      </div>
    </main>
  )
}

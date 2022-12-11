import init, {
  LevelEditor,
  WebImage,
  Keycode,
  MouseButton,
} from "./dist/index.js"

export async function run() {
  const wasm = await init()
  const [floor1, walls1, shadowsAlpha, tetrisFn2] = await Promise.all([
    loadImage("FLOOR1.PNG"),
    loadImage("WALLS1.PNG"),
    loadImage("SHADOWS_ALPHA.PNG"),
    loadFile("TETRIS.FN2"),
  ])
  const state = LevelEditor.new(floor1, walls1, shadowsAlpha, tetrisFn2)

  const canvas = document.getElementById("screen")
  canvas.width = state.screen_width()
  canvas.height = state.screen_height()
  const context = canvas.getContext("2d")

  let frameId = null
  const renderFrame = () => {
    if (frameId !== null) cancelAnimationFrame(frameId)
    frameId = requestAnimationFrame(() => {
      state.frame()
      renderToCanvas(wasm.memory.buffer, state, context)
      frameId = null
    })
  }

  // Render the initial frame
  renderFrame()

  document.addEventListener("keydown", (event) => {
    const keycode = toKeycode(event.key)
    if (keycode !== undefined) {
      event.preventDefault()
      state.key_down(keycode)
    }
    renderFrame()
  })
  canvas.addEventListener("mousemove", (event) => {
    state.mouse_move(
      (event.offsetX / canvas.clientWidth) * state.screen_width(),
      (event.offsetY / canvas.clientHeight) * state.screen_height()
    )
    renderFrame()
  })
  canvas.addEventListener("mousedown", (event) => {
    if (event.button === 0) {
      state.mouse_down(MouseButton.Left)
    }
    if (event.button === 2) {
      state.mouse_down(MouseButton.Right)
    }
    renderFrame()
  })
  canvas.addEventListener("mouseup", (event) => {
    if (event.button === 0) {
      state.mouse_up(MouseButton.Left)
    }
    if (event.button === 2) {
      state.mouse_up(MouseButton.Right)
    }
    renderFrame()
  })
}

function renderToCanvas(buffer, state, context) {
  const width = state.screen_width()
  const height = state.screen_height()
  const screen = state.screen()
  const data = new Uint8ClampedArray(buffer, screen, width * height * 4)
  context.putImageData(new ImageData(data, width, height), 0, 0)
}

async function loadImage(url) {
  const img = new Image()
  img.src = url

  await new Promise((resolve, reject) => {
    img.onload = () => {
      resolve()
    }
    img.onerror = () => {
      reject(new Error(`Unable to load image ${url}`))
    }
  })

  const canvas = document.createElement("canvas")
  canvas.width = img.width
  canvas.height = img.height
  const ctx = canvas.getContext("2d")
  ctx.drawImage(img, 0, 0)

  const imageData = ctx.getImageData(0, 0, img.width, img.height)
  return WebImage.new(
    img.width,
    img.height,
    new Uint8Array(imageData.data.buffer)
  )
}

async function loadFile(url) {
  const response = await fetch(url)
  if (!response.ok) throw new Error(`Unable to load file ${url}`)
  const arrayBuffer = await response.arrayBuffer()
  return new Uint8Array(arrayBuffer)
}

function toKeycode(key) {
  switch (key) {
    case "Escape":
      return Keycode.Escape
    case "Backspace":
      return Keycode.Backspace
    case "Enter":
      return Keycode.Return
    case "ArrowLeft":
      return Keycode.Left
    case "ArrowUp":
      return Keycode.Up
    case "ArrowRight":
      return Keycode.Right
    case "ArrowDown":
      return Keycode.Down
    case "1":
      return Keycode.Num1
    case "2":
      return Keycode.Num2
    case "a":
      return Keycode.A
    case "c":
      return Keycode.C
    case "e":
      return Keycode.E
    case "q":
      return Keycode.Q
    case "s":
      return Keycode.S
    case "w":
      return Keycode.W
    case "x":
      return Keycode.X
    case "y":
      return Keycode.Y
    case "z":
      return Keycode.Z
    case "F1":
      return Keycode.F1
    case "F2":
      return Keycode.F2
    case "F3":
      return Keycode.F3
    case "F4":
      return Keycode.F4
    case "F6":
      return Keycode.F6
    case "F7":
      return Keycode.F7
    case "F8":
      return Keycode.F8
    case "F9":
      return Keycode.F9
    case " ":
      return Keycode.Space
    case "+":
      return Keycode.Plus
    case "-":
      return Keycode.Minus
    default:
      return undefined
  }
}

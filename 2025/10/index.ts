import { readFileSync } from 'node:fs'
import { init } from 'z3-solver'


const testInput = readFileSync('inputs/test.txt', 'utf-8').split('\n')
const mainInput = readFileSync('inputs/main.txt', 'utf-8').split('\n')

interface Button {
  values: number[]
}

class Problem {
	target: boolean[]
	buttons: Button[]
	joltages: number[]
  current: number
  previousIndicators: boolean[][]
  previousJoltages: number[][]

  constructor(target: boolean[], buttons: Button[], joltages: number[]) {
    this.target = target
    this.buttons = buttons
    this.joltages = joltages
    this.current = 0

    const emptyIndicator: boolean[] = new Array(this.target.length).fill(false)
    this.previousIndicators = []
    this.previousIndicators.push(emptyIndicator)

    const emptyJoltage: number[] = new Array(this.target.length).fill(0)
    this.previousJoltages = []
    this.previousJoltages.push(emptyJoltage)
  }

  pressIndicator(value: boolean[], button: Button) {
    const altered = [...value]
    for (const v of button.values) altered[v] = !altered[v]
    if (altered.every((v, i) => v === this.target[i])) return true;
    this.previousIndicators.push(altered)
    return false
  }

  pressJoltage(value: number[], button: Button) {
    const altered = [...value]
    for (const v of button.values) altered[v]! += 1
    if (altered.every((v, i) => v === this.joltages[i])) return true;
    this.previousJoltages.push(altered)
    return false
  }

  checkIndicators() {
    const next = [...this.previousIndicators]
    this.previousIndicators = []

    for (const val of next) {
      for (const button of this.buttons) {
        if (this.pressIndicator(val, button))
          return true
      }
    }
    return false
  }

  checkJoltages() {
    const next = [...this.previousJoltages]
    this.previousJoltages = []

    for (const val of next) {
      for (const button of this.buttons) {
        if (this.pressJoltage(val, button))
          return true
      }
    }
    return false
  }

  print(value: boolean[]) {
    console.log(value.map(b => b ? '#' : '.').join(''))
  }
}

const PATTERN = /\[(.*?)\] (.*?) {(.*?)}/

function parse(input: string): Problem {
  //console.log(`Parsing ${input}`)
	const match = PATTERN.exec(input)
	if (!match || match.length < 4) throw new Error(`${input} is badly made`)

	// Parse the target
	const target = Array.from(match[1]!).map((c) => c === '#')
	const buttons = match[2]!.split(' ').map((button) => {
    const values = button.slice(1, -1).split(',').map((c) => Number(c))
    const b: Button = { values }
    return b
  })
  const joltages = match[3]!.split(',').map((v) => Number(v));
  
  return new Problem(target, buttons, joltages)
}

function solveIndicators(problem: Problem) {
  let presses = 1
  while (!problem.checkIndicators()) {
    presses += 1
    if (presses > 200) break 
  } return presses
}

function solution(inputs: string[]) {
  let partOne = 0
	for (const input of inputs) {
    if (!input) continue
    const problem = parse(input)
    partOne += solveIndicators(problem)
	}
  console.log(`Part One is ${partOne}`)
}

solution(testInput)
solution(mainInput)



const { Context } = await init()
const Z3 = Context('main')
const x = Z3.Int.const('x')
const y = Z3.Int.const('y')
const solver = new Z3.Solver()
solver.add(x.add(2).le(y.sub(10)))
solver.add(x.ge(0))

const sat = await solver.check()
if (sat === 'sat') {
  const model = solver.model()
  console.log(model.get(x), model.get(y))
} else {
  console.log('not found')
}


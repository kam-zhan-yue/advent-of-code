import { readFileSync } from 'node:fs'
import { init, type Arith, type Bool } from 'z3-solver'

const testInput = readFileSync('inputs/test.txt', 'utf-8').split('\n')
const mainInput = readFileSync('inputs/main.txt', 'utf-8').split('\n')

interface Button {
  values: number[]
  bits: boolean[]
}

class Problem {
	target: boolean[]
	buttons: Button[]
	joltages: number[]
  current: number
  previousIndicators: boolean[][]
constructor(target: boolean[], buttons: Button[], joltages: number[]) { this.target = target
    this.buttons = buttons
    this.joltages = joltages
    this.current = 0

    const emptyIndicator: boolean[] = new Array(this.target.length).fill(false)
    this.previousIndicators = []
    this.previousIndicators.push(emptyIndicator)
  }

  pressIndicator(value: boolean[], button: Button) {
    const altered = [...value]
    for (const v of button.values) altered[v] = !altered[v]
    if (altered.every((v, i) => v === this.target[i])) return true;
    this.previousIndicators.push(altered)
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
    const bits: boolean[] = new Array(target.length).fill(false)
    for (const value of values) { bits[value] = true }
    const b: Button = { values, bits }
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

async function solveJoltages(problem: Problem, Context: Awaited<ReturnType<typeof init>>['Context']) {
  const Z3 = Context('main')

  const variables = problem.buttons.map((_, i) => Z3.Int.const(`${i}`))

  const optimizer = new Z3.Optimize()
  for (const [i, joltage] of problem.joltages.entries()) {
    const equation: Arith<"main">[] = []
    for (const [j, button] of problem.buttons.entries()) {
      if (!button.bits[i]) continue
      equation.push(variables[j]!)
    }
    if (!equation) {
      throw new Error("This equation is impossible!")
    }
    if (equation.length === 0) {
      optimizer.add(equation[0]!.eq(joltage))
    } else {
      optimizer.add(equation.reduce((sum, x) => sum.add(x)).eq(joltage))
    }
  }

  for (const variable of variables) {
    optimizer.add(variable.ge(0))
  }

  const sum = variables.reduce((acc, v) => acc.add(v));
  optimizer.minimize(sum);

  const sat = await optimizer.check()
  if (sat !== 'sat')
    throw new Error("Equation was unsolvable")

  const model = optimizer.model()
  const value = variables.map((variable) => Number(model.eval(variable))).reduce((sum, val) => sum + val)
  return value
}

async function solution(inputs: string[]) {
const { Context } = await init()
  let partOne = 0
  let partTwo = 0
	for (const input of inputs) {
    if (!input) continue
    const problem = parse(input)
    partOne += solveIndicators(problem)
    partTwo += await solveJoltages(problem, Context)
	}
  console.log(`Part One is ${partOne}`)
  console.log(`Part Two is ${partTwo}`)
}

solution(testInput)
solution(mainInput)

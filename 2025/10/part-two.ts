import { init } from 'z3-solver'
const { Context } = await init()


const { Solver, Int, And } = new Context("main")

const x = Int.const('x')
const solver = new Solver()
solver.add(And(x.ge(0), x.le(9)))
console.log(await solver.check())

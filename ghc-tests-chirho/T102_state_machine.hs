-- TEST: compile_and_run
-- EXPECTED: 2\n0
module Main where
data State = Idle | Running | Done
data Event = Start | Tick | Stop
transition :: State -> Event -> State
transition Idle Start = Running
transition Running Tick = Running
transition Running Stop = Done
transition _ _ = Idle
stateToInt :: State -> Int
stateToInt Idle = 0
stateToInt Running = 1
stateToInt Done = 2
runEvents :: State -> [Event] -> State
runEvents state [] = state
runEvents state (e:es) = runEvents (transition state e) es
main :: IO ()
main = do
  print (stateToInt (runEvents Idle [Start, Tick, Tick, Stop]))
  print (stateToInt (runEvents Idle [Start, Stop, Start]))

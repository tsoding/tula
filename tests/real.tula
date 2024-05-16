trace Sum_Real { 3.4 3.5 . }

for a in Real
case Sum_Real a . -> (Sum_Real a)

for a b in Real
case (Sum_Real a) b . -> (Sum_Real a b)

for a b in Real
case (Sum_Real a b) . [a + b] . Halt

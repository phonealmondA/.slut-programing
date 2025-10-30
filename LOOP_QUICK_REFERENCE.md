# Loop Quick Reference
## Quantum Consciousness Programming Language

---

## Count Loop
**Repeat N times**
```slut
loop <> count(5) {
    speak("Hello!")
}

# With variable
times <> 10
loop <> count(times) {
    speak("Processing...")
}
```

---

## Range Loop
**Iterate with counter**
```slut
loop <> range(0, 5) as i {
    speak("Number: ~i~")
}

# Custom range
start <> 10
end <> 20
loop <> range(start, end) as num {
    speak("Value: ~num~")
}
```

---

## While Loop
**Repeat while condition is true**
```slut
count <> 0
loop <> while (count < 5) {
    speak("Count: ~count~")
    count <> calc(count, 1)
}
```

---

## Break
**Exit loop immediately**
```slut
loop <> count(10) {
    speak("Breaking!")
    break  # Only runs once
}
```

---

## Continue
**Skip to next iteration**
```slut
loop <> range(1, 10) as i {
    if <> (i % 2 == 0) <else> (true) {
        continue
    <>
        speak("Odd: ~i~")
    }
}
```

---

## Common Patterns

### Accumulator
```slut
sum <> 0
loop <> count(10) {
    sum <> calc(sum, 1)
}
```

### Search
```slut
found <> false
loop <> while (found == false) {
    result([5]) <> randomChoice([1, 2, 3, 4, 5, 6])
    if <> (result == 5) <else> (true) {
        found <> true
    <>
        speak("Searching...")
    }
}
```

### Countdown
```slut
countdown <> 10
loop <> while (countdown > 0) {
    speak("~countdown~...")
    countdown <> calc(countdown, -1)
}
speak("Liftoff!")
```

---

## ⚠️ Limitation
**Nested loops not supported yet** - keep loops at single level.

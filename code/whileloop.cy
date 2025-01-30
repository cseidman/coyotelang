let x = 1
while x < 20
    x = x + 1
    if x < 5
        continue
    endif
    print x
    if x > 10
        break
    endif
endwhile
print "Done!"
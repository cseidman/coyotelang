for i in 1 to 3
    if i == 3
        break
    endif
    print i

    for j in 10 to 20
        if j == 14
            break
        endif
        if j < 12
            continue
        endif
        print j
    endfor
endfor

/*
1
12
13
2
12
13
*/
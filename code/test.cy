let x = 6
let y = 42
let z = [10,20,30]
print z[0]
print z[1]
print z[2]
print z
print x
{
    let x = 50
    print x
    print y
    {
        let x = 14
        print x
    }
}
print x+1
print y
z[1] = 500
print z

/*let
10
20
30
[10,20,30]
6
50
42
14
7
42
[10,500,30]
*/
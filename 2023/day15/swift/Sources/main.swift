let example = """
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"""

let input = readLine()!

print(part1(input: input))

extension String {
    func trim() -> String {
        var newString = self

        while newString.first?.isWhitespace == true {
            newString = String(newString.dropFirst())
        }
        while newString.last?.isWhitespace == true {
            newString = String(newString.dropLast())
        }

        return newString
    }
}

func part1(input: String) -> UInt {
    let sequence = input.trim().split(separator: ",")

    var result: UInt = 0
    for s in sequence {
        var hash: UInt = 0
        for c in s {
            hash += UInt(c.asciiValue!)
            hash *= 17
            hash %= 256
        }
        result += hash
    }

    return result
}

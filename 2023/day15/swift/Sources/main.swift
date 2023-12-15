let input = readLine()!

let r1 = part1(input: input)
let r2 = part2(input: input)

print("Part 1: \(r1)")
print("Part 2: \(r2)")

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

struct BoxEntry {
    let label: String
    let focalLength: UInt
}

func hash<T: Collection>(_ string: T) -> UInt8 where T.Element == Character {
    var hash: UInt8 = 0
    for c in string {
        hash = hash.addingReportingOverflow(c.asciiValue!).partialValue
        hash = hash.multipliedReportingOverflow(by: 17).partialValue
    }
    return hash
}

func part1(input: String) -> UInt {
    let sequence = input.trim().split(separator: ",")

    var result: UInt = 0
    for s in sequence {
        result += UInt(hash(s))
    }

    return result
}

func part2(input: String) -> UInt {
    let sequence = input.trim().split(separator: ",")

    var map: [[BoxEntry]] = Array(repeating: [], count: 256)

    for s in sequence {
        if s.contains("-") {
            let label = s.dropLast()
            let index = Int(hash(label))
            if let j = map[index].firstIndex(where: { entry in entry.label == label }) {
                map[index].remove(at: j)
            }
        } else {
            let tmp = s.split(separator: "=", maxSplits: 1)
            let label = String(tmp[0])
            let val = UInt(tmp[1])!
            let index = Int(hash(label))

            if let j = map[index].firstIndex(where: { entry in entry.label == label }) {
                map[index].remove(at: j)
                map[index].insert(BoxEntry(label: label, focalLength: val), at: j)
            } else {
                map[index].append(BoxEntry(label: label, focalLength: val))
            }
        }
    }

    var result: UInt = 0

    for (i, box) in map.enumerated() {
        for (j, entry) in box.enumerated() {
            result += (UInt(i) + 1) * (UInt(j) + 1) * entry.focalLength
        }
    }

    return result
}

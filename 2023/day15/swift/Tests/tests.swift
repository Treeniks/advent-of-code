import XCTest
@testable import Day15

final class Day15Tests: XCTestCase {
    func testPart1() throws {
        // XCTest Documentation
        // https://developer.apple.com/documentation/xctest

        // Defining Test Cases and Test Methods
        // https://developer.apple.com/documentation/xctest/defining_test_cases_and_test_methods

        let example = """
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        """

        XCTAssertEqual(part1(input: example), 1320)
    }
}

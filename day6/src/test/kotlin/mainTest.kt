import org.junit.jupiter.api.Test

class MainTest {
    @Test
    fun testParse() {
        val input = """
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
        """.trimIndent()
        val result = parseOrbits(input)

        assert(result.contains(Orbit("COM", "B")))
        assert(result.contains(Orbit("B", "C")))
        assert(result.contains(Orbit("C", "D")))
        assert(result.contains(Orbit("D", "E")))
        assert(result.contains(Orbit("E", "F")))
        assert(result.contains(Orbit("B", "G")))
        assert(result.contains(Orbit("G", "H")))
        assert(result.contains(Orbit("D", "I")))
        assert(result.contains(Orbit("E", "J")))
        assert(result.contains(Orbit("J", "K")))
        assert(result.contains(Orbit("K", "L")))
    }
}
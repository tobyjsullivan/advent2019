import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

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

    @Test
    fun testBuildGraph() {
        val orbits = listOf(
                Orbit("AAA", "BBB"),
                Orbit("BBB", "CCC"),
                Orbit("BBB", "DDD")
        )

        val result = buildGraph(orbits)

        assert(result.nodes.contains("AAA"))
        assert(result.nodes.contains("BBB"))
        assert(result.nodes.contains("CCC"))
        assert(result.nodes.contains("DDD"))

        assert(result.edges.contains(Pair("AAA", "BBB")))
        assert(result.edges.contains(Pair("BBB", "CCC")))
        assert(result.edges.contains(Pair("BBB", "DDD")))
    }

    @Test
    fun testCountWithTerm() {
        val graph = OrbitGraph(
                setOf("AAA", "BBB", "CCC"),
                setOf(
                        Pair("AAA", "BBB"),
                        Pair("BBB", "CCC")
                ))

        val result = countOrbits("CCC", graph, "BBB")
        assertEquals(1, result)
    }

    @Test
    fun testGetChain() {

        val graph = OrbitGraph(
                setOf("AAA", "BBB", "CCC"),
                setOf(
                        Pair("AAA", "BBB"),
                        Pair("BBB", "CCC")
                ))

        val result = getChain("CCC", graph, null)

        assertEquals(listOf("BBB", "AAA"), result)
    }
}
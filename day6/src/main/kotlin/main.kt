import java.io.File
import java.lang.IllegalArgumentException

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        throw IllegalArgumentException("Filename argument must be supplied.")
    }

    val input = File(args[0]).readText(Charsets.UTF_8).trim()
    val orbits = parseOrbits(input)
    val graph = buildGraph(orbits)

    var total = 0
    for (node in graph.nodes) {
        val num = countOrbits(node, graph)
        println("Node: $node Num: $num")
        total += num
    }
    println("Total: $total")
}

data class Orbit(val orbitee: String, val orbiter: String)

data class OrbitGraph(val nodes: Set<String>, val edges: Set<Pair<String, String>>)

fun parseOrbits(input: String): List<Orbit> {
    return input.split("\n")
            .map {
                val pieces = it.trim().split(")")
                Orbit(pieces[0], pieces[1])
            }
}

fun buildGraph(orbits: List<Orbit>): OrbitGraph {
    val nodes = orbits.flatMap { orbit -> listOf(orbit.orbitee, orbit.orbiter) }.toSet()
    val edges = orbits.map { orbit -> Pair(orbit.orbitee, orbit.orbiter) }.toSet()
    return OrbitGraph(nodes, edges)
}

fun countOrbits(orbiter: String, graph: OrbitGraph): Int {
    var result = 0
    var orbitee = findOrbitee(orbiter, graph)
    while (orbitee != null) {
        result++
        orbitee = findOrbitee(orbitee, graph)
    }

    return result
}

fun findOrbitee(orbiter: String, graph: OrbitGraph): String? {
    val match = graph.edges.find { edge -> edge.second.equals(orbiter) } ?: return null
    return match.first
}
import java.io.File

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        throw IllegalArgumentException("Filename argument must be supplied.")
    }

    val input = File(args[0]).readText(Charsets.UTF_8).trim()
    val orbits = parseOrbits(input)
    val graph = buildGraph(orbits)

    val chainA = getChain("YOU", graph, null)
    val chainB = getChain("SAN", graph, null)
    var match: String? = null
    for (orb in chainA ) {
        for (orbB in chainB) {
            if  (orb == orbB) {
                match = orb
                break
            }
        }
        if (match != null) {
            break
        }
    }
    val distA = countOrbits("YOU", graph, match)
    val distB = countOrbits("SAN", graph, match)
    val indirectA = distA - 1
    val indirectB = distB - 1
    val hops = indirectA + indirectB

    println("Hops: $hops")
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

fun getChain(orbiter: String, graph: OrbitGraph, term: String?): List<String> {
    var result = mutableListOf<String>()
    var orbitee = findOrbitee(orbiter, graph)
    while (orbitee != null) {
        result.add(orbitee)
        if (orbitee == term) {
            break
        }
        orbitee = findOrbitee(orbitee, graph)
    }

    return result
}

fun countOrbits(orbiter: String, graph: OrbitGraph, term: String?): Int {
    return getChain(orbiter, graph, term).size
}

fun findOrbitee(orbiter: String, graph: OrbitGraph): String? {
    val match = graph.edges.find { edge -> edge.second.equals(orbiter) } ?: return null
    return match.first
}
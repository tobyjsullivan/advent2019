import java.lang.IllegalArgumentException

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        throw IllegalArgumentException("Filename argument must be supplied.")
    }
    println("Hello: " + args[0])
}

data class Orbit(val orbitee: String, val orbiter: String)

data class OrbitGraph(val nodes: List<String>, val edges: List<Pair<String, String>>)

fun parseOrbits(input: String): List<Orbit> {
    return input.split("\n")
            .map {
                val pieces = it.split(")")
                Orbit(pieces[0], pieces[1])
            }
}
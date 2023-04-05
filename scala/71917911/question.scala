object GfG {
    // Main method
    def main(args:Array[String]) = {
        val castes = Seq(
          "Mark Wahlberg",
          "Leonardo DiCaprio",
          "Kate Mara",
          "Ben Affleck"
        )

        val movies = Seq(
          (
            "Departed",
            "South Boston cop Billy Costigan (Leonardo DiCaprio) goes under cover, detective (Mark Wahlberg) who ...",
            2006
          ),
          (
            "Shooter",
            "A top Marine sniper, Bob Lee Swagger (Mark Wahlberg), who leaves the military, Sarah (Kate Mara) the widow of...",
            2007
          ),
          (
            "Triple Frontier",
            "Former Special Forces operative Tom (Ben Affleck) and his team ......",
            2019
          )
        )

        var t1 = System.nanoTime
        val res1 = castes.flatMap(r => {
          movies
            .filter(x => x._2.contains(r))
            .map(s => {
              (r, s._1)
            })
        })
        var duration = (System.nanoTime - t1) / 1e9d
        println(res1)
        println(duration)

        t1 = System.nanoTime
        val result = castes.foldLeft(
          List[(String, String)]()
        )(
          (r1, caste) => {
            r1 ::: movies.foldLeft(
              List[(String, String)]()
            )(
              (r2, m) => {
                if (m._2.contains(caste)) r2 :+ (caste, m._1) else r2
              }
            )
          }
        )

        duration = (System.nanoTime - t1) / 1e9d
        println(result)
        println(duration)
    }
}

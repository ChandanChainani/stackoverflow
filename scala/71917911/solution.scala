object HelloWorld {
  def test1(castes: Seq[String], movies: Seq[(String, String, Int)]): Seq[(String, String, String)] = {
    castes.flatMap(r => {
      movies
      .map(s => {
        (r, s._1, s._2)
      }).filter(x => x._3 contains x._1)
    })
  }

  def test2(castes: Seq[String], movies: Seq[(String, String, Int)]): Seq[(String, String)] = {
    castes.foldLeft(
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
	}
 
	def main(args: Array[String]): Unit = {
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
    val r1 = test2(castes, movies)
    var duration = (System.nanoTime - t1) / 1e9d
    println(duration)
    println(r1)
    t1 = System.nanoTime
    val r2 = test1(castes, movies)
    duration = (System.nanoTime - t1) / 1e9d
    println(duration)
    println(r2)
  }
}

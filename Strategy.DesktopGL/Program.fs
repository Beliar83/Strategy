namespace Strategy.Game

module Program =


    [<EntryPoint>]
    let main argv =
        use game = new StrategyGame()
        game.Run()
        0 // return an integer exit code

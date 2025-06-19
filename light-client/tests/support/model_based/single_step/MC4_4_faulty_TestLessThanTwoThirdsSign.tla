---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n3", "n4" },
                  VS |-> { "n1", "n2", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n4" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 1399]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n4" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 1399,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1399
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1399
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 3 to State1 *)
State1 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n3", "n4" },
                  VS |-> { "n1", "n2", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n4" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 1399]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n2", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 4
          :> [Commits |-> { "n1", "n3" },
            header |->
              [NextVS |-> {"n4"},
                VS |-> { "n1", "n3" },
                height |-> 4,
                lastCommit |-> { "n3", "n4" },
                time |-> 4]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 1399,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> { "n1", "n3" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n3" },
                    height |-> 4,
                    lastCommit |-> { "n3", "n4" },
                    time |-> 4]],
            now |-> 1399,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateUnverified"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> { "n1", "n3" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n3" },
            height |-> 4,
            lastCommit |-> { "n3", "n4" },
            time |-> 4]]
    /\ prevNow = 1399
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 3 to State2 *)
State2 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n3", "n4" },
                  VS |-> { "n1", "n2", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n4" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 1399]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 4
            :> [Commits |-> { "n1", "n3" },
              header |->
                [NextVS |-> {"n4"},
                  VS |-> { "n1", "n3" },
                  height |-> 4,
                  lastCommit |-> { "n3", "n4" },
                  time |-> 4]])
        @@ 3
          :> [Commits |-> { "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n3" },
                VS |-> { "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n4" },
                time |-> 3]]
    /\ history
      = (0
            :> [current |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]],
              now |-> 1399,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]]
          @@ 1
            :> [current |->
                [Commits |-> { "n1", "n3" },
                  header |->
                    [NextVS |-> {"n4"},
                      VS |-> { "n1", "n3" },
                      height |-> 4,
                      lastCommit |-> { "n3", "n4" },
                      time |-> 4]],
              now |-> 1399,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 2
          :> [current |->
              [Commits |-> { "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n3" },
                    VS |-> { "n3", "n4" },
                    height |-> 3,
                    lastCommit |-> { "n1", "n2", "n4" },
                    time |-> 3]],
            now |-> 1400,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified")
        @@ 3 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1400
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n3" },
            VS |-> { "n3", "n4" },
            height |-> 3,
            lastCommit |-> { "n1", "n2", "n4" },
            time |-> 3]]
    /\ prevNow = 1400
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 0 to State3 *)
State3 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n3", "n4" },
                  VS |-> { "n1", "n2", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n4" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 1399]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 4
              :> [Commits |-> { "n1", "n3" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n3" },
                    height |-> 4,
                    lastCommit |-> { "n3", "n4" },
                    time |-> 4]])
          @@ 3
            :> [Commits |-> { "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n3" },
                  VS |-> { "n3", "n4" },
                  height |-> 3,
                  lastCommit |-> { "n1", "n2", "n4" },
                  time |-> 3]])
        @@ 5
          :> [Commits |-> {},
            header |->
              [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> {"n4"},
                height |-> 5,
                lastCommit |-> { "n1", "n3" },
                time |-> 1399]]
    /\ history
      = ((0
              :> [current |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n4" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]],
                now |-> 1399,
                verdict |-> "SUCCESS",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n4" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]]
            @@ 1
              :> [current |->
                  [Commits |-> { "n1", "n3" },
                    header |->
                      [NextVS |-> {"n4"},
                        VS |-> { "n1", "n3" },
                        height |-> 4,
                        lastCommit |-> { "n3", "n4" },
                        time |-> 4]],
                now |-> 1399,
                verdict |-> "NOT_ENOUGH_TRUST",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n4" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]])
          @@ 2
            :> [current |->
                [Commits |-> { "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n3" },
                      VS |-> { "n3", "n4" },
                      height |-> 3,
                      lastCommit |-> { "n1", "n2", "n4" },
                      time |-> 3]],
              now |-> 1400,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 3
          :> [current |->
              [Commits |-> {},
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4" },
                    VS |-> {"n4"},
                    height |-> 5,
                    lastCommit |-> { "n1", "n3" },
                    time |-> 1399]],
            now |-> 1400,
            verdict |-> "INVALID",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateUnverified")
          @@ 3 :> "StateUnverified")
        @@ 2 :> "StateFailed"
    /\ nextHeight = 2
    /\ now = 1400
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> {},
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 1399]]
    /\ prevNow = 1400
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "finishedFailure"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  Cardinality((DOMAIN fetchedLightBlocks)) = 4
    /\ Apalache!Skolem((\E s$2 \in DOMAIN history:
      history[s$2]["verified"]["header"]["time"] + 1400 > history[s$2]["now"]
        /\ Apalache!Skolem((\E commits$2 \in SUBSET ({ "n1", "n2", "n3", "n4" }):
          Apalache!Skolem((\E t_12$1 \in SUBSET ({ "n1", "n2", "n3", "n4" }):
            Apalache!Skolem((\E t_10$1 \in SUBSET ({ "n1", "n2", "n3", "n4" }):
              Apalache!Skolem((\E t_z$1 \in SUBSET ({ "n1", "n2", "n3", "n4" }):
                Apalache!Skolem((\E t_y$1 \in SUBSET ({ "n1", "n2", "n3", "n4" }):
                  Apalache!Skolem((\E t_x$1 \in Int:
                    Apalache!Skolem((\E t_w$1 \in 1 .. 5:
                      3 * Cardinality(commits$2)
                          < 2
                            * Cardinality(history[s$2]["current"]["header"][
                              "VS"
                            ])
                        /\ LET ref$5 ==
                          blockchain[
                            history[s$2]["current"]["header"]["height"]
                          ]
                        IN
                        LET lastCommit$6 ==
                          IF history[s$2]["current"]["header"]["height"] < 5
                          THEN blockchain[
                            (history[s$2]["current"]["header"]["height"] + 1)
                          ][
                            "lastCommit"
                          ]
                          ELSE blockchain[
                            history[s$2]["current"]["header"]["height"]
                          ][
                            "VS"
                          ]
                        IN
                        [header |->
                            [height |-> t_w$1,
                              time |-> t_x$1,
                              lastCommit |-> t_y$1,
                              VS |-> t_z$1,
                              NextVS |-> t_10$1],
                          Commits |-> t_12$1]
                          = [header |-> ref$5, Commits |-> lastCommit$6]
                        /\ history[s$2]["current"]
                          = [
                            [header |->
                                [height |-> t_w$1,
                                  time |-> t_x$1,
                                  lastCommit |-> t_y$1,
                                  VS |-> t_z$1,
                                  NextVS |-> t_10$1],
                              Commits |-> t_12$1] EXCEPT
                              ![<<"Commits">>] = commits$2
                          ]))))))))))))))))

================================================================================
(* Created by Apalache on Thu Jun 12 17:02:17 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

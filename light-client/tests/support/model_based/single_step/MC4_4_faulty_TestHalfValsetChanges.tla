---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n4" },
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 6]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> {"n4"},
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 6,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 6
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 6
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 5 to State1 *)
State1 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n4" },
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 6]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> {"n4"},
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 4
          :> [Commits |-> { "n1", "n3" },
            header |->
              [NextVS |-> { "n1", "n2", "n4" },
                VS |-> { "n1", "n3" },
                height |-> 4,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 5]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 6,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> { "n1", "n3" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n3" },
                    height |-> 4,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 5]],
            now |-> 6,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
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
          [NextVS |-> { "n1", "n2", "n4" },
            VS |-> { "n1", "n3" },
            height |-> 4,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 5]]
    /\ prevNow = 6
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 5 to State2 *)
State2 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n4" },
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 6]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 4
            :> [Commits |-> { "n1", "n3" },
              header |->
                [NextVS |-> { "n1", "n2", "n4" },
                  VS |-> { "n1", "n3" },
                  height |-> 4,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 5]])
        @@ 3
          :> [Commits |-> { "n1", "n2", "n3" },
            header |->
              [NextVS |-> { "n1", "n3" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3]]
    /\ history
      = (0
            :> [current |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n4"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]],
              now |-> 6,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n4"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]]
          @@ 1
            :> [current |->
                [Commits |-> { "n1", "n3" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n4" },
                      VS |-> { "n1", "n3" },
                      height |-> 4,
                      lastCommit |-> { "n1", "n2", "n3" },
                      time |-> 5]],
              now |-> 6,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n4"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 2
          :> [current |->
              [Commits |-> { "n1", "n2", "n3" },
                header |->
                  [NextVS |-> { "n1", "n3" },
                    VS |-> { "n1", "n2", "n3" },
                    height |-> 3,
                    lastCommit |-> {"n4"},
                    time |-> 3]],
            now |-> 1400,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified")
        @@ 3 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1401
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3" },
        header |->
          [NextVS |-> { "n1", "n3" },
            VS |-> { "n1", "n2", "n3" },
            height |-> 3,
            lastCommit |-> {"n4"},
            time |-> 3]]
    /\ prevNow = 1400
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
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
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n3" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n4" },
              VS |-> { "n1", "n3" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n3" },
            time |-> 6]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 4
              :> [Commits |-> { "n1", "n3" },
                header |->
                  [NextVS |-> { "n1", "n2", "n4" },
                    VS |-> { "n1", "n3" },
                    height |-> 4,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 5]])
          @@ 3
            :> [Commits |-> { "n1", "n2", "n3" },
              header |->
                [NextVS |-> { "n1", "n3" },
                  VS |-> { "n1", "n2", "n3" },
                  height |-> 3,
                  lastCommit |-> {"n4"},
                  time |-> 3]])
        @@ 2
          :> [Commits |-> { "n1", "n2", "n4" },
            header |->
              [NextVS |-> { "n1", "n2", "n3" },
                VS |-> {"n4"},
                height |-> 2,
                lastCommit |-> { "n2", "n3", "n4" },
                time |-> 2]]
    /\ history
      = ((0
              :> [current |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> {"n4"},
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]],
                now |-> 6,
                verdict |-> "SUCCESS",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> {"n4"},
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]]
            @@ 1
              :> [current |->
                  [Commits |-> { "n1", "n3" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n4" },
                        VS |-> { "n1", "n3" },
                        height |-> 4,
                        lastCommit |-> { "n1", "n2", "n3" },
                        time |-> 5]],
                now |-> 6,
                verdict |-> "NOT_ENOUGH_TRUST",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> {"n4"},
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]])
          @@ 2
            :> [current |->
                [Commits |-> { "n1", "n2", "n3" },
                  header |->
                    [NextVS |-> { "n1", "n3" },
                      VS |-> { "n1", "n2", "n3" },
                      height |-> 3,
                      lastCommit |-> {"n4"},
                      time |-> 3]],
              now |-> 1400,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n4"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 3
          :> [current |->
              [Commits |-> { "n1", "n2", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> {"n4"},
                    height |-> 2,
                    lastCommit |-> { "n2", "n3", "n4" },
                    time |-> 2]],
            now |-> 1401,
            verdict |-> "INVALID",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateUnverified")
          @@ 3 :> "StateUnverified")
        @@ 2 :> "StateFailed"
    /\ nextHeight = 2
    /\ now = 1401
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> {"n4"},
            height |-> 2,
            lastCommit |-> { "n2", "n3", "n4" },
            time |-> 2]]
    /\ prevNow = 1401
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "finishedFailure"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  Cardinality((DOMAIN fetchedLightBlocks)) = 4
    /\ Apalache!Skolem((\E s1$2 \in DOMAIN history:
      Apalache!Skolem((\E s2$2 \in DOMAIN history:
        s2$2 = s1$2 + 1
          /\ Apalache!ConstCardinality((Cardinality(history[s1$2]["current"][
            "header"
          ][
            "VS"
          ])
            >= 3))
          /\ 2
            * Cardinality({
              t_k$1 \in history[s1$2]["current"]["header"]["VS"]:
                t_k$1 \in history[s2$2]["current"]["header"]["VS"]
            })
            < Cardinality(history[s1$2]["current"]["header"]["VS"])))))

================================================================================
(* Created by Apalache on Thu Jun 12 17:01:19 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

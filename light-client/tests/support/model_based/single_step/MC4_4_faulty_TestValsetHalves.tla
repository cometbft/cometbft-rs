---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 1395])
            @@ 3
              :> [NextVS |-> { "n1", "n2" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> { "n2", "n3", "n4" },
                time |-> 1396])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n3", "n4" },
              VS |-> { "n1", "n2" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 1397])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n2" },
            time |-> 1398]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n2", "n3", "n4" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 1398,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1398
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1398
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n2", "n3", "n4" },
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
                :> [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 1395])
            @@ 3
              :> [NextVS |-> { "n1", "n2" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> { "n2", "n3", "n4" },
                time |-> 1396])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n3", "n4" },
              VS |-> { "n1", "n2" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 1397])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n2" },
            time |-> 1398]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n2", "n3", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 4
          :> [Commits |-> { "n1", "n2" },
            header |->
              [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> { "n1", "n2" },
                height |-> 4,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 1397]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 1398,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> { "n1", "n2" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4" },
                    VS |-> { "n1", "n2" },
                    height |-> 4,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 1397]],
            now |-> 1398,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1398
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> { "n1", "n2" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2" },
            height |-> 4,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 1397]]
    /\ prevNow = 1398
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n2", "n3", "n4" },
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
                :> [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 1395])
            @@ 3
              :> [NextVS |-> { "n1", "n2" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> { "n2", "n3", "n4" },
                time |-> 1396])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n3", "n4" },
              VS |-> { "n1", "n2" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 1397])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n2" },
            time |-> 1398]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 4
            :> [Commits |-> { "n1", "n2" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2" },
                  height |-> 4,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 1397]])
        @@ 2
          :> [Commits |-> { "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n2", "n3" },
                VS |-> { "n2", "n3", "n4" },
                height |-> 2,
                lastCommit |-> { "n2", "n3", "n4" },
                time |-> 1395]]
    /\ history
      = (0
            :> [current |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n2", "n3", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]],
              now |-> 1398,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n2", "n3", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]]
          @@ 1
            :> [current |->
                [Commits |-> { "n1", "n2" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3", "n4" },
                      VS |-> { "n1", "n2" },
                      height |-> 4,
                      lastCommit |-> { "n1", "n2", "n3" },
                      time |-> 1397]],
              now |-> 1398,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n2", "n3", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 2
          :> [current |->
              [Commits |-> { "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n2", "n3", "n4" },
                    height |-> 2,
                    lastCommit |-> { "n2", "n3", "n4" },
                    time |-> 1395]],
            now |-> 1398,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n2", "n3", "n4" },
            height |-> 2,
            lastCommit |-> { "n2", "n3", "n4" },
            time |-> 1395]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified") @@ 2 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 2797
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n2", "n3", "n4" },
            height |-> 2,
            lastCommit |-> { "n2", "n3", "n4" },
            time |-> 1395]]
    /\ prevNow = 1398
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 1 to State3 *)
State3 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 1395])
            @@ 3
              :> [NextVS |-> { "n1", "n2" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 3,
                lastCommit |-> { "n2", "n3", "n4" },
                time |-> 1396])
          @@ 4
            :> [NextVS |-> { "n1", "n2", "n3", "n4" },
              VS |-> { "n1", "n2" },
              height |-> 4,
              lastCommit |-> { "n1", "n2", "n3" },
              time |-> 1397])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 5,
            lastCommit |-> { "n1", "n2" },
            time |-> 1398]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 4
              :> [Commits |-> { "n1", "n2" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4" },
                    VS |-> { "n1", "n2" },
                    height |-> 4,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 1397]])
          @@ 2
            :> [Commits |-> { "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 1395]])
        @@ 3
          :> [Commits |-> {},
            header |->
              [NextVS |-> {"n1"},
                VS |-> { "n1", "n3", "n4" },
                height |-> 3,
                lastCommit |-> {"n3"},
                time |-> 1397]]
    /\ history
      = ((0
              :> [current |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n2", "n3", "n4" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]],
                now |-> 1398,
                verdict |-> "SUCCESS",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n2", "n3", "n4" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]]
            @@ 1
              :> [current |->
                  [Commits |-> { "n1", "n2" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n3", "n4" },
                        VS |-> { "n1", "n2" },
                        height |-> 4,
                        lastCommit |-> { "n1", "n2", "n3" },
                        time |-> 1397]],
                now |-> 1398,
                verdict |-> "NOT_ENOUGH_TRUST",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n2", "n3", "n4" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]])
          @@ 2
            :> [current |->
                [Commits |-> { "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3" },
                      VS |-> { "n2", "n3", "n4" },
                      height |-> 2,
                      lastCommit |-> { "n2", "n3", "n4" },
                      time |-> 1395]],
              now |-> 1398,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n2", "n3", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 3
          :> [current |->
              [Commits |-> {},
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n3", "n4" },
                    height |-> 3,
                    lastCommit |-> {"n3"},
                    time |-> 1397]],
            now |-> 2797,
            verdict |-> "INVALID",
            verified |->
              [Commits |-> { "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n2", "n3", "n4" },
                    height |-> 2,
                    lastCommit |-> { "n2", "n3", "n4" },
                    time |-> 1395]]]
    /\ latestVerified
      = [Commits |-> { "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n2", "n3", "n4" },
            height |-> 2,
            lastCommit |-> { "n2", "n3", "n4" },
            time |-> 1395]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateUnverified")
          @@ 2 :> "StateVerified")
        @@ 3 :> "StateFailed"
    /\ nextHeight = 3
    /\ now = 2797
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> {},
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n3", "n4" },
            height |-> 3,
            lastCommit |-> {"n3"},
            time |-> 1397]]
    /\ prevNow = 2797
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |-> { "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n2", "n3", "n4" },
            height |-> 2,
            lastCommit |-> { "n2", "n3", "n4" },
            time |-> 1395]]
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
            >= 4))
          /\ Cardinality(history[s1$2]["current"]["header"]["VS"])
            = 2 * Cardinality(history[s2$2]["current"]["header"]["VS"])))))

================================================================================
(* Created by Apalache on Thu Jun 12 17:01:48 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {"n4"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> {"n2"},
                  VS |-> { "n1", "n2" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> {"n2"},
                height |-> 3,
                lastCommit |-> { "n1", "n2" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n3" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> {"n2"},
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n3" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 1398,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2" },
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
          [NextVS |-> { "n1", "n2" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1398
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 4 to State1 *)
State1 ==
  Faulty = {"n4"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> {"n2"},
                  VS |-> { "n1", "n2" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> {"n2"},
                height |-> 3,
                lastCommit |-> { "n1", "n2" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n3" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> {"n2"},
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n3" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n2" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 5
          :> [Commits |-> {"n4"},
            header |->
              [NextVS |-> { "n1", "n4" },
                VS |-> {"n4"},
                height |-> 5,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 1397]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 1398,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> {"n4"},
                header |->
                  [NextVS |-> { "n1", "n4" },
                    VS |-> {"n4"},
                    height |-> 5,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 1397]],
            now |-> 1398,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateUnverified"
    /\ nextHeight = 3
    /\ now = 1398
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {"n4"},
        header |->
          [NextVS |-> { "n1", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 1397]]
    /\ prevNow = 1398
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 5 to State2 *)
State2 ==
  Faulty = {"n4"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> {"n2"},
                  VS |-> { "n1", "n2" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> {"n2"},
                height |-> 3,
                lastCommit |-> { "n1", "n2" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n3" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> {"n2"},
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n3" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 5
            :> [Commits |-> {"n4"},
              header |->
                [NextVS |-> { "n1", "n4" },
                  VS |-> {"n4"},
                  height |-> 5,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 1397]])
        @@ 3
          :> [Commits |-> {"n2"},
            header |->
              [NextVS |-> {"n1"},
                VS |-> {"n2"},
                height |-> 3,
                lastCommit |-> { "n1", "n2" },
                time |-> 3]]
    /\ history
      = (0
            :> [current |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]],
              now |-> 1398,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]]
          @@ 1
            :> [current |->
                [Commits |-> {"n4"},
                  header |->
                    [NextVS |-> { "n1", "n4" },
                      VS |-> {"n4"},
                      height |-> 5,
                      lastCommit |-> { "n1", "n2", "n3" },
                      time |-> 1397]],
              now |-> 1398,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 2
          :> [current |->
              [Commits |-> {"n2"},
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> {"n2"},
                    height |-> 3,
                    lastCommit |-> { "n1", "n2" },
                    time |-> 3]],
            now |-> 1398,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> {"n2"},
        header |->
          [NextVS |-> {"n1"},
            VS |-> {"n2"},
            height |-> 3,
            lastCommit |-> { "n1", "n2" },
            time |-> 3]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified") @@ 3 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1404
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> {"n2"},
        header |->
          [NextVS |-> {"n1"},
            VS |-> {"n2"},
            height |-> 3,
            lastCommit |-> { "n1", "n2" },
            time |-> 3]]
    /\ prevNow = 1398
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 0 to State3 *)
State3 ==
  Faulty = {"n4"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> {"n2"},
                  VS |-> { "n1", "n2" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> {"n2"},
                height |-> 3,
                lastCommit |-> { "n1", "n2" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n3" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> {"n2"},
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n3" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 5
              :> [Commits |-> {"n4"},
                header |->
                  [NextVS |-> { "n1", "n4" },
                    VS |-> {"n4"},
                    height |-> 5,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 1397]])
          @@ 3
            :> [Commits |-> {"n2"},
              header |->
                [NextVS |-> {"n1"},
                  VS |-> {"n2"},
                  height |-> 3,
                  lastCommit |-> { "n1", "n2" },
                  time |-> 3]])
        @@ 4
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n3" },
                VS |-> {"n1"},
                height |-> 4,
                lastCommit |-> {"n2"},
                time |-> 4]]
    /\ history
      = ((0
              :> [current |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]],
                now |-> 1398,
                verdict |-> "SUCCESS",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]]
            @@ 1
              :> [current |->
                  [Commits |-> {"n4"},
                    header |->
                      [NextVS |-> { "n1", "n4" },
                        VS |-> {"n4"},
                        height |-> 5,
                        lastCommit |-> { "n1", "n2", "n3" },
                        time |-> 1397]],
                now |-> 1398,
                verdict |-> "NOT_ENOUGH_TRUST",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]])
          @@ 2
            :> [current |->
                [Commits |-> {"n2"},
                  header |->
                    [NextVS |-> {"n1"},
                      VS |-> {"n2"},
                      height |-> 3,
                      lastCommit |-> { "n1", "n2" },
                      time |-> 3]],
              now |-> 1398,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 3
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n3" },
                    VS |-> {"n1"},
                    height |-> 4,
                    lastCommit |-> {"n2"},
                    time |-> 4]],
            now |-> 1404,
            verdict |-> "INVALID",
            verified |->
              [Commits |-> {"n2"},
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> {"n2"},
                    height |-> 3,
                    lastCommit |-> { "n1", "n2" },
                    time |-> 3]]]
    /\ latestVerified
      = [Commits |-> {"n2"},
        header |->
          [NextVS |-> {"n1"},
            VS |-> {"n2"},
            height |-> 3,
            lastCommit |-> { "n1", "n2" },
            time |-> 3]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateFailed") @@ 3 :> "StateVerified")
        @@ 4 :> "StateFailed"
    /\ nextHeight = 4
    /\ now = 1404
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n3" },
            VS |-> {"n1"},
            height |-> 4,
            lastCommit |-> {"n2"},
            time |-> 4]]
    /\ prevNow = 1404
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |-> {"n2"},
        header |->
          [NextVS |-> {"n1"},
            VS |-> {"n2"},
            height |-> 3,
            lastCommit |-> { "n1", "n2" },
            time |-> 3]]
    /\ state = "finishedFailure"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  Cardinality((DOMAIN fetchedLightBlocks)) = 4
    /\ Apalache!Skolem((\E s$2 \in DOMAIN history:
      history[s$2]["now"] > history[s$2]["verified"]["header"]["time"] + 1400
        /\ history[s$2]["current"]["header"]["time"] < history[s$2]["now"]))

================================================================================
(* Created by Apalache on Thu Jun 12 17:01:05 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

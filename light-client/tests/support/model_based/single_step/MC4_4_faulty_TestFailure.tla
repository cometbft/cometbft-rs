---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n2", "n3" },
                  VS |-> {"n1"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 3])
            @@ 3
              :> [NextVS |-> { "n2", "n4" },
                VS |-> { "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n1"},
                time |-> 4])
          @@ 4
            :> [NextVS |-> { "n2", "n4" },
              VS |-> { "n2", "n4" },
              height |-> 4,
              lastCommit |-> { "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n2", "n4" },
            time |-> 6]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> {"n1"},
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 1400,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1400
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 1 to State1 *)
State1 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n2", "n3" },
                  VS |-> {"n1"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 3])
            @@ 3
              :> [NextVS |-> { "n2", "n4" },
                VS |-> { "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n1"},
                time |-> 4])
          @@ 4
            :> [NextVS |-> { "n2", "n4" },
              VS |-> { "n2", "n4" },
              height |-> 4,
              lastCommit |-> { "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n2", "n4" },
            time |-> 6]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> {"n1"},
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 4
          :> [Commits |-> { "n2", "n4" },
            header |->
              [NextVS |-> { "n2", "n4" },
                VS |-> { "n2", "n4" },
                height |-> 4,
                lastCommit |-> { "n2", "n3" },
                time |-> 5]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> { "n2", "n4" },
                header |->
                  [NextVS |-> { "n2", "n4" },
                    VS |-> { "n2", "n4" },
                    height |-> 4,
                    lastCommit |-> { "n2", "n3" },
                    time |-> 5]],
            now |-> 1400,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateUnverified"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> { "n2", "n4" },
        header |->
          [NextVS |-> { "n2", "n4" },
            VS |-> { "n2", "n4" },
            height |-> 4,
            lastCommit |-> { "n2", "n3" },
            time |-> 5]]
    /\ prevNow = 1400
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 1 to State2 *)
State2 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n2", "n3" },
                  VS |-> {"n1"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 3])
            @@ 3
              :> [NextVS |-> { "n2", "n4" },
                VS |-> { "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n1"},
                time |-> 4])
          @@ 4
            :> [NextVS |-> { "n2", "n4" },
              VS |-> { "n2", "n4" },
              height |-> 4,
              lastCommit |-> { "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n2", "n4" },
            time |-> 6]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 4
            :> [Commits |-> { "n2", "n4" },
              header |->
                [NextVS |-> { "n2", "n4" },
                  VS |-> { "n2", "n4" },
                  height |-> 4,
                  lastCommit |-> { "n2", "n3" },
                  time |-> 5]])
        @@ 3
          :> [Commits |-> { "n2", "n3" },
            header |->
              [NextVS |-> { "n2", "n4" },
                VS |-> { "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n1"},
                time |-> 4]]
    /\ history
      = (0
            :> [current |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n1"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]],
              now |-> 1400,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n1"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]]
          @@ 1
            :> [current |->
                [Commits |-> { "n2", "n4" },
                  header |->
                    [NextVS |-> { "n2", "n4" },
                      VS |-> { "n2", "n4" },
                      height |-> 4,
                      lastCommit |-> { "n2", "n3" },
                      time |-> 5]],
              now |-> 1400,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n1"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 2
          :> [current |->
              [Commits |-> { "n2", "n3" },
                header |->
                  [NextVS |-> { "n2", "n4" },
                    VS |-> { "n2", "n3" },
                    height |-> 3,
                    lastCommit |-> {"n1"},
                    time |-> 4]],
            now |-> 1400,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified")
        @@ 3 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1403
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n2", "n3" },
        header |->
          [NextVS |-> { "n2", "n4" },
            VS |-> { "n2", "n3" },
            height |-> 3,
            lastCommit |-> {"n1"},
            time |-> 4]]
    /\ prevNow = 1400
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
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
                :> [NextVS |-> {"n1"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n2", "n3" },
                  VS |-> {"n1"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 3])
            @@ 3
              :> [NextVS |-> { "n2", "n4" },
                VS |-> { "n2", "n3" },
                height |-> 3,
                lastCommit |-> {"n1"},
                time |-> 4])
          @@ 4
            :> [NextVS |-> { "n2", "n4" },
              VS |-> { "n2", "n4" },
              height |-> 4,
              lastCommit |-> { "n2", "n3" },
              time |-> 5])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n2", "n4" },
            height |-> 5,
            lastCommit |-> { "n2", "n4" },
            time |-> 6]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 4
              :> [Commits |-> { "n2", "n4" },
                header |->
                  [NextVS |-> { "n2", "n4" },
                    VS |-> { "n2", "n4" },
                    height |-> 4,
                    lastCommit |-> { "n2", "n3" },
                    time |-> 5]])
          @@ 3
            :> [Commits |-> { "n2", "n3" },
              header |->
                [NextVS |-> { "n2", "n4" },
                  VS |-> { "n2", "n3" },
                  height |-> 3,
                  lastCommit |-> {"n1"},
                  time |-> 4]])
        @@ 2
          :> [Commits |-> {"n1"},
            header |->
              [NextVS |-> { "n2", "n3" },
                VS |-> {"n1"},
                height |-> 2,
                lastCommit |-> { "n1", "n2", "n3", "n4" },
                time |-> 3]]
    /\ history
      = ((0
              :> [current |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> {"n1"},
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]],
                now |-> 1400,
                verdict |-> "SUCCESS",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> {"n1"},
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]]
            @@ 1
              :> [current |->
                  [Commits |-> { "n2", "n4" },
                    header |->
                      [NextVS |-> { "n2", "n4" },
                        VS |-> { "n2", "n4" },
                        height |-> 4,
                        lastCommit |-> { "n2", "n3" },
                        time |-> 5]],
                now |-> 1400,
                verdict |-> "NOT_ENOUGH_TRUST",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> {"n1"},
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]])
          @@ 2
            :> [current |->
                [Commits |-> { "n2", "n3" },
                  header |->
                    [NextVS |-> { "n2", "n4" },
                      VS |-> { "n2", "n3" },
                      height |-> 3,
                      lastCommit |-> {"n1"},
                      time |-> 4]],
              now |-> 1400,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> {"n1"},
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 3
          :> [current |->
              [Commits |-> {"n1"},
                header |->
                  [NextVS |-> { "n2", "n3" },
                    VS |-> {"n1"},
                    height |-> 2,
                    lastCommit |-> { "n1", "n2", "n3", "n4" },
                    time |-> 3]],
            now |-> 1403,
            verdict |-> "INVALID",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateUnverified")
          @@ 3 :> "StateUnverified")
        @@ 2 :> "StateFailed"
    /\ nextHeight = 2
    /\ now = 1403
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> {"n1"},
        header |->
          [NextVS |-> { "n2", "n3" },
            VS |-> {"n1"},
            height |-> 2,
            lastCommit |-> { "n1", "n2", "n3", "n4" },
            time |-> 3]]
    /\ prevNow = 1403
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "finishedFailure"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  state = "finishedFailure" /\ Cardinality((DOMAIN fetchedLightBlocks)) = 4

================================================================================
(* Created by Apalache on Thu Jun 12 16:59:32 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {"n3"}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n4" },
              height |-> 4,
              lastCommit |-> { "n1", "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n4" },
            time |-> 5]
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
          now |-> 1398,
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
    /\ now = 1398
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1398
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

(* Transition 1 to State1 *)
State1 ==
  Faulty = {"n3"}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n4" },
              height |-> 4,
              lastCommit |-> { "n1", "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n4" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> {"n4"},
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 5
          :> [Commits |-> {"n3"},
            header |->
              [NextVS |-> { "n1", "n4" },
                VS |-> {"n3"},
                height |-> 5,
                lastCommit |-> {"n1"},
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
            now |-> 1398,
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
              [Commits |-> {"n3"},
                header |->
                  [NextVS |-> { "n1", "n4" },
                    VS |-> {"n3"},
                    height |-> 5,
                    lastCommit |-> {"n1"},
                    time |-> 5]],
            now |-> 1398,
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
    /\ now = 1398
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {"n3"},
        header |->
          [NextVS |-> { "n1", "n4" },
            VS |-> {"n3"},
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]]
    /\ prevNow = 1398
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

(* Transition 4 to State2 *)
State2 ==
  Faulty = {"n3"}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n4" },
              height |-> 4,
              lastCommit |-> { "n1", "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n4" },
            time |-> 5]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 5
            :> [Commits |-> {"n3"},
              header |->
                [NextVS |-> { "n1", "n4" },
                  VS |-> {"n3"},
                  height |-> 5,
                  lastCommit |-> {"n1"},
                  time |-> 5]])
        @@ 3
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
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
              now |-> 1398,
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
                [Commits |-> {"n3"},
                  header |->
                    [NextVS |-> { "n1", "n4" },
                      VS |-> {"n3"},
                      height |-> 5,
                      lastCommit |-> {"n1"},
                      time |-> 5]],
              now |-> 1398,
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
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 3,
                    lastCommit |-> {"n4"},
                    time |-> 3]],
            now |-> 1398,
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
          [NextVS |-> { "n1", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 3,
            lastCommit |-> {"n4"},
            time |-> 3]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified") @@ 3 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1402
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 3,
            lastCommit |-> {"n4"},
            time |-> 3]]
    /\ prevNow = 1398
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

(* Transition 4 to State3 *)
State3 ==
  Faulty = {"n3"}
    /\ blockchain
      = (((1
                :> [NextVS |-> {"n4"},
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> {"n4"},
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 3,
                lastCommit |-> {"n4"},
                time |-> 3])
          @@ 4
            :> [NextVS |-> {"n4"},
              VS |-> { "n1", "n4" },
              height |-> 4,
              lastCommit |-> { "n1", "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> {"n4"},
            height |-> 5,
            lastCommit |-> { "n1", "n4" },
            time |-> 5]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 5
              :> [Commits |-> {"n3"},
                header |->
                  [NextVS |-> { "n1", "n4" },
                    VS |-> {"n3"},
                    height |-> 5,
                    lastCommit |-> {"n1"},
                    time |-> 5]])
          @@ 3
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 3,
                  lastCommit |-> {"n4"},
                  time |-> 3]])
        @@ 4
          :> [Commits |-> { "n1", "n4" },
            header |->
              [NextVS |-> {"n4"},
                VS |-> { "n1", "n4" },
                height |-> 4,
                lastCommit |-> { "n1", "n3", "n4" },
                time |-> 4]]
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
                now |-> 1398,
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
                  [Commits |-> {"n3"},
                    header |->
                      [NextVS |-> { "n1", "n4" },
                        VS |-> {"n3"},
                        height |-> 5,
                        lastCommit |-> {"n1"},
                        time |-> 5]],
                now |-> 1398,
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
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n4" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 3,
                      lastCommit |-> {"n4"},
                      time |-> 3]],
              now |-> 1398,
              verdict |-> "SUCCESS",
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
              [Commits |-> { "n1", "n4" },
                header |->
                  [NextVS |-> {"n4"},
                    VS |-> { "n1", "n4" },
                    height |-> 4,
                    lastCommit |-> { "n1", "n3", "n4" },
                    time |-> 4]],
            now |-> 1402,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 3,
                    lastCommit |-> {"n4"},
                    time |-> 3]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n4" },
            height |-> 4,
            lastCommit |-> { "n1", "n3", "n4" },
            time |-> 4]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateVerified") @@ 3 :> "StateVerified")
        @@ 4 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1402
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> { "n1", "n4" },
        header |->
          [NextVS |-> {"n4"},
            VS |-> { "n1", "n4" },
            height |-> 4,
            lastCommit |-> { "n1", "n3", "n4" },
            time |-> 4]]
    /\ prevNow = 1402
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 3,
            lastCommit |-> {"n4"},
            time |-> 3]]
    /\ state = "finishedSuccess"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  state = "finishedSuccess" /\ Cardinality((DOMAIN fetchedLightBlocks)) = 4

================================================================================
(* Created by Apalache on Thu Jun 12 16:59:26 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

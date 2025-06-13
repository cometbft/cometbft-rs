---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n3" },
                  VS |-> { "n1", "n2", "n3" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> { "n1", "n3" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n3", "n4" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> { "n1", "n3" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n3", "n4" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 1400,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
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
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
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
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n3" },
                  VS |-> { "n1", "n2", "n3" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> { "n1", "n3" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n3", "n4" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> { "n1", "n3" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n3", "n4" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n2", "n3" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 4
          :> [Commits |-> {"n1"},
            header |->
              [NextVS |-> { "n3", "n4" },
                VS |-> {"n1"},
                height |-> 4,
                lastCommit |-> { "n1", "n3" },
                time |-> 4]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> {"n1"},
                header |->
                  [NextVS |-> { "n3", "n4" },
                    VS |-> {"n1"},
                    height |-> 4,
                    lastCommit |-> { "n1", "n3" },
                    time |-> 4]],
            now |-> 1400,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1400
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {"n1"},
        header |->
          [NextVS |-> { "n3", "n4" },
            VS |-> {"n1"},
            height |-> 4,
            lastCommit |-> { "n1", "n3" },
            time |-> 4]]
    /\ prevNow = 1400
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 0 to State2 *)
State2 ==
  Faulty = {}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n3" },
                  VS |-> { "n1", "n2", "n3" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> { "n1", "n3" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n3", "n4" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> { "n1", "n3" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n3", "n4" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]
          @@ 4
            :> [Commits |-> {"n1"},
              header |->
                [NextVS |-> { "n3", "n4" },
                  VS |-> {"n1"},
                  height |-> 4,
                  lastCommit |-> { "n1", "n3" },
                  time |-> 4]])
        @@ 2
          :> [Commits |-> { "n1", "n2", "n3" },
            header |->
              [NextVS |-> { "n1", "n3" },
                VS |-> { "n1", "n2", "n3" },
                height |-> 2,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 2]]
    /\ history
      = (0
            :> [current |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]],
              now |-> 1400,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]]
          @@ 1
            :> [current |->
                [Commits |-> {"n1"},
                  header |->
                    [NextVS |-> { "n3", "n4" },
                      VS |-> {"n1"},
                      height |-> 4,
                      lastCommit |-> { "n1", "n3" },
                      time |-> 4]],
              now |-> 1400,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3" },
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
                    height |-> 2,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 2]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3" },
        header |->
          [NextVS |-> { "n1", "n3" },
            VS |-> { "n1", "n2", "n3" },
            height |-> 2,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 2]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 4 :> "StateUnverified") @@ 2 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3" },
        header |->
          [NextVS |-> { "n1", "n3" },
            VS |-> { "n1", "n2", "n3" },
            height |-> 2,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 2]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3" },
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
                :> [NextVS |-> { "n1", "n2", "n3" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n3" },
                  VS |-> { "n1", "n2", "n3" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> {"n1"},
                VS |-> { "n1", "n3" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n3", "n4" },
              VS |-> {"n1"},
              height |-> 4,
              lastCommit |-> { "n1", "n3" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n3", "n4" },
            height |-> 5,
            lastCommit |-> {"n1"},
            time |-> 5]
    /\ fetchedLightBlocks
      = ((1
              :> [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]
            @@ 4
              :> [Commits |-> {"n1"},
                header |->
                  [NextVS |-> { "n3", "n4" },
                    VS |-> {"n1"},
                    height |-> 4,
                    lastCommit |-> { "n1", "n3" },
                    time |-> 4]])
          @@ 2
            :> [Commits |-> { "n1", "n2", "n3" },
              header |->
                [NextVS |-> { "n1", "n3" },
                  VS |-> { "n1", "n2", "n3" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3" },
                  time |-> 2]])
        @@ 3
          :> [Commits |-> { "n1", "n3" },
            header |->
              [NextVS |-> {"n1"},
                VS |-> { "n1", "n3" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3]]
    /\ history
      = ((0
              :> [current |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n3" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]],
                now |-> 1400,
                verdict |-> "SUCCESS",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n3" },
                        VS |-> { "n1", "n2", "n3", "n4" },
                        height |-> 1,
                        lastCommit |-> {},
                        time |-> 1]]]
            @@ 1
              :> [current |->
                  [Commits |-> {"n1"},
                    header |->
                      [NextVS |-> { "n3", "n4" },
                        VS |-> {"n1"},
                        height |-> 4,
                        lastCommit |-> { "n1", "n3" },
                        time |-> 4]],
                now |-> 1400,
                verdict |-> "NOT_ENOUGH_TRUST",
                verified |->
                  [Commits |-> { "n1", "n2", "n3", "n4" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n3" },
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
                      height |-> 2,
                      lastCommit |-> { "n1", "n2", "n3" },
                      time |-> 2]],
              now |-> 1400,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |-> { "n1", "n2", "n3", "n4" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3" },
                      VS |-> { "n1", "n2", "n3", "n4" },
                      height |-> 1,
                      lastCommit |-> {},
                      time |-> 1]]])
        @@ 3
          :> [current |->
              [Commits |-> { "n1", "n3" },
                header |->
                  [NextVS |-> {"n1"},
                    VS |-> { "n1", "n3" },
                    height |-> 3,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 3]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3" },
                header |->
                  [NextVS |-> { "n1", "n3" },
                    VS |-> { "n1", "n2", "n3" },
                    height |-> 2,
                    lastCommit |-> { "n1", "n2", "n3" },
                    time |-> 2]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n3" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n3" },
            height |-> 3,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 3]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 4 :> "StateUnverified")
          @@ 2 :> "StateVerified")
        @@ 3 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 1400
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> { "n1", "n3" },
        header |->
          [NextVS |-> {"n1"},
            VS |-> { "n1", "n3" },
            height |-> 3,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 3]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3" },
        header |->
          [NextVS |-> { "n1", "n3" },
            VS |-> { "n1", "n2", "n3" },
            height |-> 2,
            lastCommit |-> { "n1", "n2", "n3" },
            time |-> 2]]
    /\ state = "working"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  Cardinality((DOMAIN fetchedLightBlocks)) = 4
    /\ (\A s1$2 \in DOMAIN history:
      \A s2$2 \in DOMAIN history:
        s1$2 = s2$2
          \/ ~(history[s1$2]["current"]["header"]["VS"]
            = history[s2$2]["current"]["header"]["VS"]))

================================================================================
(* Created by Apalache on Thu Jun 12 17:01:12 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

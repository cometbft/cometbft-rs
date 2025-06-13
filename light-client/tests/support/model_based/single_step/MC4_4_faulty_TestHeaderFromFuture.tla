---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {"n3"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n2" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 4,
              lastCommit |-> { "n2", "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2" },
            height |-> 5,
            lastCommit |-> { "n1", "n2", "n3", "n4" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |-> { "n1", "n2", "n3", "n4" },
          header |->
            [NextVS |-> { "n1", "n2", "n3", "n4" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 5,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |-> { "n1", "n2", "n3", "n4" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 4
    /\ now = 5
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 5
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 4 to State1 *)
State1 ==
  Faulty = {"n3"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n1", "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n1", "n2" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 4,
              lastCommit |-> { "n2", "n3", "n4" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2" },
            height |-> 5,
            lastCommit |-> { "n1", "n2", "n3", "n4" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> { "n1", "n2", "n3", "n4" },
            header |->
              [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> { "n1", "n2", "n3", "n4" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 2
          :> [Commits |-> {},
            header |->
              [NextVS |-> {},
                VS |-> {},
                height |-> 2,
                lastCommit |-> { "n1", "n2", "n3", "n4" },
                time |-> 6]]
    /\ history
      = 0
          :> [current |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 5,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> {},
                header |->
                  [NextVS |-> {},
                    VS |-> {},
                    height |-> 2,
                    lastCommit |-> { "n1", "n2", "n3", "n4" },
                    time |-> 6]],
            now |-> 5,
            verdict |-> "INVALID",
            verified |->
              [Commits |-> { "n1", "n2", "n3", "n4" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4" },
                    VS |-> { "n1", "n2", "n3", "n4" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 4 :> "StateFailed"
    /\ nextHeight = 4
    /\ now = 5
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {},
        header |->
          [NextVS |-> {},
            VS |-> {},
            height |-> 2,
            lastCommit |-> { "n1", "n2", "n3", "n4" },
            time |-> 6]]
    /\ prevNow = 5
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "finishedFailure"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  Apalache!Skolem((\E s$2 \in DOMAIN history:
      history[s$2]["now"] < history[s$2]["current"]["header"]["time"]
        /\ history[s$2]["now"]
          < history[s$2]["verified"]["header"]["time"] + 1400))

================================================================================
(* Created by Apalache on Thu Jun 12 17:00:58 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

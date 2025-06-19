---------------------------- MODULE counterexample ----------------------------

EXTENDS MC4_4_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {"n4"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> { "n1", "n2" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3", "n4" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 4,
              lastCommit |-> { "n1", "n2" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n2", "n3" },
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
          now |-> 1400,
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
    /\ now = 1400
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |-> { "n1", "n2", "n3", "n4" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n1", "n2", "n3", "n4" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1400
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

(* Transition 3 to State1 *)
State1 ==
  Faulty = {"n4"}
    /\ blockchain
      = (((1
                :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]
              @@ 2
                :> [NextVS |-> { "n1", "n2" },
                  VS |-> { "n1", "n2", "n3", "n4" },
                  height |-> 2,
                  lastCommit |-> { "n2", "n3", "n4" },
                  time |-> 2])
            @@ 3
              :> [NextVS |-> { "n1", "n2", "n3", "n4" },
                VS |-> { "n1", "n2" },
                height |-> 3,
                lastCommit |-> { "n1", "n2", "n3", "n4" },
                time |-> 3])
          @@ 4
            :> [NextVS |-> { "n2", "n3" },
              VS |-> { "n1", "n2", "n3", "n4" },
              height |-> 4,
              lastCommit |-> { "n1", "n2" },
              time |-> 4])
        @@ 5
          :> [NextVS |-> { "n1", "n2", "n3", "n4" },
            VS |-> { "n2", "n3" },
            height |-> 5,
            lastCommit |-> { "n1", "n2", "n3", "n4" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> {"n4"},
            header |->
              [NextVS |-> {},
                VS |-> {"n4"},
                height |-> 1,
                lastCommit |-> { "n1", "n3", "n4" },
                time |-> 4]]
        @@ 1
          :> [Commits |-> {"n4"},
            header |->
              [NextVS |-> {},
                VS |-> {"n4"},
                height |-> 1,
                lastCommit |-> { "n1", "n3", "n4" },
                time |-> 4]]
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
            now |-> 1400,
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
              [Commits |-> {"n4"},
                header |->
                  [NextVS |-> {},
                    VS |-> {"n4"},
                    height |-> 1,
                    lastCommit |-> { "n1", "n3", "n4" },
                    time |-> 4]],
            now |-> 1400,
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
    /\ now = 1400
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {"n4"},
        header |->
          [NextVS |-> {},
            VS |-> {"n4"},
            height |-> 1,
            lastCommit |-> { "n1", "n3", "n4" },
            time |-> 4]]
    /\ prevNow = 1400
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
      history[s$2]["current"]["header"]["height"]
          <= history[s$2]["verified"]["header"]["height"]
        /\ ~(history[s$2]["current"]["header"]
          = history[s$2]["verified"]["header"])
        /\ history[s$2]["current"]["header"]["time"]
          > history[s$2]["verified"]["header"]["time"]
        /\ history[s$2]["current"]["header"]["time"] < history[s$2]["now"]
        /\ history[s$2]["verified"]["header"]["time"] + 1400
          > history[s$2]["now"]
        /\ ~(history[s$2]["current"]["Commits"] = {})
        /\ history[s$2]["current"]["Commits"]
          \subseteq history[s$2]["current"]["header"]["VS"]))

================================================================================
(* Created by Apalache on Thu Jun 12 17:00:53 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

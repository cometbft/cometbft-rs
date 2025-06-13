---------------------------- MODULE counterexample ----------------------------

EXTENDS MC10_3_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = { "n10", "n7" }
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n1", "n5" },
                VS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                height |-> 2,
                lastCommit |-> { "n1", "n2", "n4", "n6", "n7", "n8", "n9" },
                time |-> 1398])
          @@ 3
            :> [NextVS |-> { "n10", "n2", "n3", "n4", "n5", "n6", "n9" },
              VS |-> { "n1", "n5" },
              height |-> 3,
              lastCommit |-> { "n10", "n2", "n4", "n8", "n9" },
              time |-> 1399])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n10", "n2", "n3", "n4", "n5", "n6", "n9" },
            height |-> 4,
            lastCommit |-> { "n1", "n5" },
            time |-> 1400]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |->
            { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
          header |->
            [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
              VS |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              height |-> 1,
              lastCommit |-> {},
              time |-> 1]]
    /\ history
      = 0
        :> [current |->
            [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                  VS |->
                    { "n1",
                      "n10",
                      "n2",
                      "n3",
                      "n4",
                      "n5",
                      "n6",
                      "n7",
                      "n8",
                      "n9" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]],
          now |-> 1400,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                  VS |->
                    { "n1",
                      "n10",
                      "n2",
                      "n3",
                      "n4",
                      "n5",
                      "n6",
                      "n7",
                      "n8",
                      "n9" },
                  height |-> 1,
                  lastCommit |-> {},
                  time |-> 1]]]
    /\ latestVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 5 to State1 *)
State1 ==
  Faulty = { "n10", "n7" }
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n1", "n5" },
                VS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                height |-> 2,
                lastCommit |-> { "n1", "n2", "n4", "n6", "n7", "n8", "n9" },
                time |-> 1398])
          @@ 3
            :> [NextVS |-> { "n10", "n2", "n3", "n4", "n5", "n6", "n9" },
              VS |-> { "n1", "n5" },
              height |-> 3,
              lastCommit |-> { "n10", "n2", "n4", "n8", "n9" },
              time |-> 1399])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n10", "n2", "n3", "n4", "n5", "n6", "n9" },
            height |-> 4,
            lastCommit |-> { "n1", "n5" },
            time |-> 1400]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            header |->
              [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 3
          :> [Commits |-> {"n7"},
            header |->
              [NextVS |-> { "n1", "n2", "n7", "n8" },
                VS |-> { "n2", "n5" },
                height |-> 3,
                lastCommit |-> { "n2", "n5", "n6", "n7", "n8" },
                time |-> 1401]]
    /\ history
      = 0
          :> [current |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                    VS |->
                      { "n1",
                        "n10",
                        "n2",
                        "n3",
                        "n4",
                        "n5",
                        "n6",
                        "n7",
                        "n8",
                        "n9" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                    VS |->
                      { "n1",
                        "n10",
                        "n2",
                        "n3",
                        "n4",
                        "n5",
                        "n6",
                        "n7",
                        "n8",
                        "n9" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
        @@ 1
          :> [current |->
              [Commits |-> {"n7"},
                header |->
                  [NextVS |-> { "n1", "n2", "n7", "n8" },
                    VS |-> { "n2", "n5" },
                    height |-> 3,
                    lastCommit |-> { "n2", "n5", "n6", "n7", "n8" },
                    time |-> 1401]],
            now |-> 1400,
            verdict |-> "INVALID",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
                    VS |->
                      { "n1",
                        "n10",
                        "n2",
                        "n3",
                        "n4",
                        "n5",
                        "n6",
                        "n7",
                        "n8",
                        "n9" },
                    height |-> 1,
                    lastCommit |-> {},
                    time |-> 1]]]
    /\ latestVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 3 :> "StateFailed"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {"n7"},
        header |->
          [NextVS |-> { "n1", "n2", "n7", "n8" },
            VS |-> { "n2", "n5" },
            height |-> 3,
            lastCommit |-> { "n2", "n5", "n6", "n7", "n8" },
            time |-> 1401]]
    /\ prevNow = 1400
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n10", "n2", "n4", "n5", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
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
(* Created by Apalache on Thu Jun 12 16:46:26 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

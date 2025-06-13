---------------------------- MODULE counterexample ----------------------------

EXTENDS MC10_3_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {"n2"}
    /\ blockchain
      = ((1
              :> [NextVS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n3", "n5", "n6", "n8", "n9" },
                VS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
                height |-> 2,
                lastCommit |-> { "n1", "n10", "n2", "n3", "n4", "n8", "n9" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n10", "n2", "n3", "n5", "n6", "n7" },
              VS |-> { "n3", "n5", "n6", "n8", "n9" },
              height |-> 3,
              lastCommit |-> { "n10", "n2", "n3", "n5", "n6", "n7", "n8" },
              time |-> 3])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n10", "n2", "n3", "n5", "n6", "n7" },
            height |-> 4,
            lastCommit |-> { "n3", "n5", "n6", "n8", "n9" },
            time |-> 4]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |->
            { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
          header |->
            [NextVS |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
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
                [NextVS |->
                    { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
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
          now |-> 4,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |->
                    { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
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
          [NextVS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 4
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 4
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 5 to State1 *)
State1 ==
  Faulty = {"n2"}
    /\ blockchain
      = ((1
              :> [NextVS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n3", "n5", "n6", "n8", "n9" },
                VS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
                height |-> 2,
                lastCommit |-> { "n1", "n10", "n2", "n3", "n4", "n8", "n9" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n10", "n2", "n3", "n5", "n6", "n7" },
              VS |-> { "n3", "n5", "n6", "n8", "n9" },
              height |-> 3,
              lastCommit |-> { "n10", "n2", "n3", "n5", "n6", "n7", "n8" },
              time |-> 3])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n10", "n2", "n3", "n5", "n6", "n7" },
            height |-> 4,
            lastCommit |-> { "n3", "n5", "n6", "n8", "n9" },
            time |-> 4]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |-> {"n2"},
            header |->
              [NextVS |-> {},
                VS |-> {"n2"},
                height |-> 1,
                lastCommit |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                time |-> 2]]
        @@ 1
          :> [Commits |-> {"n2"},
            header |->
              [NextVS |-> {},
                VS |-> {"n2"},
                height |-> 1,
                lastCommit |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                time |-> 2]]
    /\ history
      = 0
          :> [current |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |->
                      { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
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
            now |-> 4,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |->
                      { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
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
              [Commits |-> {"n2"},
                header |->
                  [NextVS |-> {},
                    VS |-> {"n2"},
                    height |-> 1,
                    lastCommit |->
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
                    time |-> 2]],
            now |-> 4,
            verdict |-> "INVALID",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |->
                      { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
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
          [NextVS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 3 :> "StateFailed"
    /\ nextHeight = 3
    /\ now = 4
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> {"n2"},
        header |->
          [NextVS |-> {},
            VS |-> {"n2"},
            height |-> 1,
            lastCommit |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            time |-> 2]]
    /\ prevNow = 4
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
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
(* Created by Apalache on Thu Jun 12 16:46:19 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

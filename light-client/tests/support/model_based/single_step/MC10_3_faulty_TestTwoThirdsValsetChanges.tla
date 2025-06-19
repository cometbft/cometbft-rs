---------------------------- MODULE counterexample ----------------------------

EXTENDS MC10_3_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = { "n10", "n6", "n7", "n9" }
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n1", "n2", "n3", "n6" },
                VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                height |-> 2,
                lastCommit |-> { "n1", "n10", "n2", "n3", "n4", "n6", "n7" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n4", "n5", "n6", "n7", "n8" },
              VS |-> { "n1", "n2", "n3", "n6" },
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n4", "n5", "n6", "n8" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n4", "n5", "n6", "n7", "n8" },
            height |-> 4,
            lastCommit |-> { "n1", "n3", "n6" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |->
            { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
          header |->
            [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
                [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
          now |-> 5,
          verdict |-> "SUCCESS",
          verified |->
            [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 5
    /\ nprobes = 0
    /\ prevCurrent
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ prevNow = 5
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 0 to State1 *)
State1 ==
  Faulty = { "n10", "n6", "n7", "n9" }
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n1", "n2", "n3", "n6" },
                VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                height |-> 2,
                lastCommit |-> { "n1", "n10", "n2", "n3", "n4", "n6", "n7" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n4", "n5", "n6", "n7", "n8" },
              VS |-> { "n1", "n2", "n3", "n6" },
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n4", "n5", "n6", "n8" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n4", "n5", "n6", "n7", "n8" },
            height |-> 4,
            lastCommit |-> { "n1", "n3", "n6" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            header |->
              [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 4
          :> [Commits |-> { "n10", "n7", "n9" },
            header |->
              [NextVS |-> { "n10", "n4", "n5", "n7" },
                VS |-> { "n10", "n7", "n9" },
                height |-> 4,
                lastCommit |-> { "n1", "n2", "n5", "n6", "n7", "n8" },
                time |-> 3]]
    /\ history
      = 0
          :> [current |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
            now |-> 5,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
              [Commits |-> { "n10", "n7", "n9" },
                header |->
                  [NextVS |-> { "n10", "n4", "n5", "n7" },
                    VS |-> { "n10", "n7", "n9" },
                    height |-> 4,
                    lastCommit |-> { "n1", "n2", "n5", "n6", "n7", "n8" },
                    time |-> 3]],
            now |-> 5,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 3 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1402
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> { "n10", "n7", "n9" },
        header |->
          [NextVS |-> { "n10", "n4", "n5", "n7" },
            VS |-> { "n10", "n7", "n9" },
            height |-> 4,
            lastCommit |-> { "n1", "n2", "n5", "n6", "n7", "n8" },
            time |-> 3]]
    /\ prevNow = 5
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 3 to State2 *)
State2 ==
  Faulty = { "n10", "n6", "n7", "n9" }
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n1", "n2", "n3", "n6" },
                VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
                height |-> 2,
                lastCommit |-> { "n1", "n10", "n2", "n3", "n4", "n6", "n7" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n4", "n5", "n6", "n7", "n8" },
              VS |-> { "n1", "n2", "n3", "n6" },
              height |-> 3,
              lastCommit |-> { "n1", "n2", "n4", "n5", "n6", "n8" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n4", "n5", "n6", "n7", "n8" },
            height |-> 4,
            lastCommit |-> { "n1", "n3", "n6" },
            time |-> 5]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
                  time |-> 1]]
          @@ 4
            :> [Commits |-> { "n10", "n7", "n9" },
              header |->
                [NextVS |-> { "n10", "n4", "n5", "n7" },
                  VS |-> { "n10", "n7", "n9" },
                  height |-> 4,
                  lastCommit |-> { "n1", "n2", "n5", "n6", "n7", "n8" },
                  time |-> 3]])
        @@ 2
          :> [Commits |-> { "n6", "n7" },
            header |->
              [NextVS |-> { "n1", "n2", "n3", "n6", "n8", "n9" },
                VS |-> { "n2", "n4" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n9" },
                time |-> 2]]
    /\ history
      = (0
            :> [current |->
                [Commits |->
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
                  header |->
                    [NextVS |->
                        { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
              now |-> 5,
              verdict |-> "SUCCESS",
              verified |->
                [Commits |->
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
                  header |->
                    [NextVS |->
                        { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
                [Commits |-> { "n10", "n7", "n9" },
                  header |->
                    [NextVS |-> { "n10", "n4", "n5", "n7" },
                      VS |-> { "n10", "n7", "n9" },
                      height |-> 4,
                      lastCommit |-> { "n1", "n2", "n5", "n6", "n7", "n8" },
                      time |-> 3]],
              now |-> 5,
              verdict |-> "NOT_ENOUGH_TRUST",
              verified |->
                [Commits |->
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
                  header |->
                    [NextVS |->
                        { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
                      time |-> 1]]])
        @@ 2
          :> [current |->
              [Commits |-> { "n6", "n7" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n6", "n8", "n9" },
                    VS |-> { "n2", "n4" },
                    height |-> 2,
                    lastCommit |-> { "n10", "n3", "n4", "n9" },
                    time |-> 2]],
            now |-> 1402,
            verdict |-> "INVALID",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
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
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 3 :> "StateUnverified") @@ 2 :> "StateFailed"
    /\ nextHeight = 2
    /\ now = 1402
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n6", "n7" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n6", "n8", "n9" },
            VS |-> { "n2", "n4" },
            height |-> 2,
            lastCommit |-> { "n10", "n3", "n4", "n9" },
            time |-> 2]]
    /\ prevNow = 1402
    /\ prevVerdict = "INVALID"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8", "n9" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "finishedFailure"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  Cardinality((DOMAIN fetchedLightBlocks)) = 3
    /\ Apalache!Skolem((\E s1$2 \in DOMAIN history:
      Apalache!Skolem((\E s2$2 \in DOMAIN history:
        s2$2 = s1$2 + 1
          /\ Apalache!ConstCardinality((Cardinality(history[s1$2]["current"][
            "header"
          ][
            "VS"
          ])
            >= 3))
          /\ 3
            * Cardinality({
              t_k$1 \in history[s2$2]["current"]["header"]["VS"]:
                ~(t_k$1 \in history[s1$2]["current"]["header"]["VS"])
            })
            = 2 * Cardinality(history[s1$2]["current"]["header"]["VS"])))))

================================================================================
(* Created by Apalache on Thu Jun 12 16:48:05 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

---------------------------- MODULE counterexample ----------------------------

EXTENDS MC10_3_faulty

(* Constant initialization state *)
ConstInit == TRUE

(* Initial state *)
State0 ==
  Faulty = {"n7"}
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n3", "n4", "n6" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n2", "n6", "n7", "n8" },
                VS |-> { "n1", "n3", "n4", "n6" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
              VS |-> { "n2", "n6", "n7", "n8" },
              height |-> 3,
              lastCommit |-> { "n1", "n3", "n4", "n6" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            height |-> 4,
            lastCommit |-> { "n2", "n6", "n7" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
        :> [Commits |->
            { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
          header |->
            [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                [NextVS |-> { "n1", "n3", "n4", "n6" },
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
          [NextVS |-> { "n1", "n3", "n4", "n6" },
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
          [NextVS |-> { "n1", "n3", "n4", "n6" },
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
          [NextVS |-> { "n1", "n3", "n4", "n6" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 1 to State1 *)
State1 ==
  Faulty = {"n7"}
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n3", "n4", "n6" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n2", "n6", "n7", "n8" },
                VS |-> { "n1", "n3", "n4", "n6" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
              VS |-> { "n2", "n6", "n7", "n8" },
              height |-> 3,
              lastCommit |-> { "n1", "n3", "n4", "n6" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            height |-> 4,
            lastCommit |-> { "n2", "n6", "n7" },
            time |-> 5]
    /\ fetchedLightBlocks
      = 1
          :> [Commits |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            header |->
              [NextVS |-> { "n1", "n3", "n4", "n6" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]]
        @@ 3
          :> [Commits |-> { "n6", "n7", "n8" },
            header |->
              [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                VS |-> { "n2", "n6", "n7", "n8" },
                height |-> 3,
                lastCommit |-> { "n1", "n3", "n4", "n6" },
                time |-> 4]]
    /\ history
      = 0
          :> [current |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                  [NextVS |-> { "n1", "n3", "n4", "n6" },
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
              [Commits |-> { "n6", "n7", "n8" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                    VS |-> { "n2", "n6", "n7", "n8" },
                    height |-> 3,
                    lastCommit |-> { "n1", "n3", "n4", "n6" },
                    time |-> 4]],
            now |-> 1400,
            verdict |-> "NOT_ENOUGH_TRUST",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n3", "n4", "n6" },
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
          [NextVS |-> { "n1", "n3", "n4", "n6" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ lightBlockStatus = 1 :> "StateVerified" @@ 3 :> "StateUnverified"
    /\ nextHeight = 2
    /\ now = 1400
    /\ nprobes = 1
    /\ prevCurrent
      = [Commits |-> { "n6", "n7", "n8" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            VS |-> { "n2", "n6", "n7", "n8" },
            height |-> 3,
            lastCommit |-> { "n1", "n3", "n4", "n6" },
            time |-> 4]]
    /\ prevNow = 1400
    /\ prevVerdict = "NOT_ENOUGH_TRUST"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n3", "n4", "n6" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 4 to State2 *)
State2 ==
  Faulty = {"n7"}
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n3", "n4", "n6" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n2", "n6", "n7", "n8" },
                VS |-> { "n1", "n3", "n4", "n6" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
              VS |-> { "n2", "n6", "n7", "n8" },
              height |-> 3,
              lastCommit |-> { "n1", "n3", "n4", "n6" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            height |-> 4,
            lastCommit |-> { "n2", "n6", "n7" },
            time |-> 5]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |-> { "n1", "n3", "n4", "n6" },
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
          @@ 3
            :> [Commits |-> { "n6", "n7", "n8" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                  VS |-> { "n2", "n6", "n7", "n8" },
                  height |-> 3,
                  lastCommit |-> { "n1", "n3", "n4", "n6" },
                  time |-> 4]])
        @@ 2
          :> [Commits |-> { "n1", "n3", "n6" },
            header |->
              [NextVS |-> { "n2", "n6", "n7", "n8" },
                VS |-> { "n1", "n3", "n4", "n6" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
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
                    [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                    [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                [Commits |-> { "n6", "n7", "n8" },
                  header |->
                    [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                      VS |-> { "n2", "n6", "n7", "n8" },
                      height |-> 3,
                      lastCommit |-> { "n1", "n3", "n4", "n6" },
                      time |-> 4]],
              now |-> 1400,
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
                    [NextVS |-> { "n1", "n3", "n4", "n6" },
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
              [Commits |-> { "n1", "n3", "n6" },
                header |->
                  [NextVS |-> { "n2", "n6", "n7", "n8" },
                    VS |-> { "n1", "n3", "n4", "n6" },
                    height |-> 2,
                    lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                    time |-> 2]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                header |->
                  [NextVS |-> { "n1", "n3", "n4", "n6" },
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
      = [Commits |-> { "n1", "n3", "n6" },
        header |->
          [NextVS |-> { "n2", "n6", "n7", "n8" },
            VS |-> { "n1", "n3", "n4", "n6" },
            height |-> 2,
            lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
            time |-> 2]]
    /\ lightBlockStatus
      = (1 :> "StateVerified" @@ 3 :> "StateUnverified") @@ 2 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 2
    /\ prevCurrent
      = [Commits |-> { "n1", "n3", "n6" },
        header |->
          [NextVS |-> { "n2", "n6", "n7", "n8" },
            VS |-> { "n1", "n3", "n4", "n6" },
            height |-> 2,
            lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
            time |-> 2]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |->
          { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
        header |->
          [NextVS |-> { "n1", "n3", "n4", "n6" },
            VS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            height |-> 1,
            lastCommit |-> {},
            time |-> 1]]
    /\ state = "working"

(* Transition 5 to State3 *)
State3 ==
  Faulty = {"n7"}
    /\ blockchain
      = ((1
              :> [NextVS |-> { "n1", "n3", "n4", "n6" },
                VS |->
                  { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
                height |-> 1,
                lastCommit |-> {},
                time |-> 1]
            @@ 2
              :> [NextVS |-> { "n2", "n6", "n7", "n8" },
                VS |-> { "n1", "n3", "n4", "n6" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                time |-> 2])
          @@ 3
            :> [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
              VS |-> { "n2", "n6", "n7", "n8" },
              height |-> 3,
              lastCommit |-> { "n1", "n3", "n4", "n6" },
              time |-> 4])
        @@ 4
          :> [NextVS |->
              { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
            VS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            height |-> 4,
            lastCommit |-> { "n2", "n6", "n7" },
            time |-> 5]
    /\ fetchedLightBlocks
      = (1
            :> [Commits |->
                { "n1", "n10", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9" },
              header |->
                [NextVS |-> { "n1", "n3", "n4", "n6" },
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
          @@ 3
            :> [Commits |-> { "n6", "n7", "n8" },
              header |->
                [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                  VS |-> { "n2", "n6", "n7", "n8" },
                  height |-> 3,
                  lastCommit |-> { "n1", "n3", "n4", "n6" },
                  time |-> 4]])
        @@ 2
          :> [Commits |-> { "n1", "n3", "n6" },
            header |->
              [NextVS |-> { "n2", "n6", "n7", "n8" },
                VS |-> { "n1", "n3", "n4", "n6" },
                height |-> 2,
                lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                time |-> 2]]
    /\ history
      = ((0
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
                      [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                      [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                  [Commits |-> { "n6", "n7", "n8" },
                    header |->
                      [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                        VS |-> { "n2", "n6", "n7", "n8" },
                        height |-> 3,
                        lastCommit |-> { "n1", "n3", "n4", "n6" },
                        time |-> 4]],
                now |-> 1400,
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
                      [NextVS |-> { "n1", "n3", "n4", "n6" },
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
                [Commits |-> { "n1", "n3", "n6" },
                  header |->
                    [NextVS |-> { "n2", "n6", "n7", "n8" },
                      VS |-> { "n1", "n3", "n4", "n6" },
                      height |-> 2,
                      lastCommit |->
                        { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                      time |-> 2]],
              now |-> 1400,
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
                    [NextVS |-> { "n1", "n3", "n4", "n6" },
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
        @@ 3
          :> [current |->
              [Commits |-> { "n6", "n7", "n8" },
                header |->
                  [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
                    VS |-> { "n2", "n6", "n7", "n8" },
                    height |-> 3,
                    lastCommit |-> { "n1", "n3", "n4", "n6" },
                    time |-> 4]],
            now |-> 1400,
            verdict |-> "SUCCESS",
            verified |->
              [Commits |-> { "n1", "n3", "n6" },
                header |->
                  [NextVS |-> { "n2", "n6", "n7", "n8" },
                    VS |-> { "n1", "n3", "n4", "n6" },
                    height |-> 2,
                    lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
                    time |-> 2]]]
    /\ latestVerified
      = [Commits |-> { "n6", "n7", "n8" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            VS |-> { "n2", "n6", "n7", "n8" },
            height |-> 3,
            lastCommit |-> { "n1", "n3", "n4", "n6" },
            time |-> 4]]
    /\ lightBlockStatus
      = ((1 :> "StateVerified" @@ 3 :> "StateVerified") @@ 2 :> "StateVerified")
        @@ 3 :> "StateVerified"
    /\ nextHeight = 3
    /\ now = 1400
    /\ nprobes = 3
    /\ prevCurrent
      = [Commits |-> { "n6", "n7", "n8" },
        header |->
          [NextVS |-> { "n1", "n2", "n3", "n4", "n5", "n6", "n8" },
            VS |-> { "n2", "n6", "n7", "n8" },
            height |-> 3,
            lastCommit |-> { "n1", "n3", "n4", "n6" },
            time |-> 4]]
    /\ prevNow = 1400
    /\ prevVerdict = "SUCCESS"
    /\ prevVerified
      = [Commits |-> { "n1", "n3", "n6" },
        header |->
          [NextVS |-> { "n2", "n6", "n7", "n8" },
            VS |-> { "n1", "n3", "n4", "n6" },
            height |-> 2,
            lastCommit |-> { "n10", "n3", "n4", "n5", "n6", "n8", "n9" },
            time |-> 2]]
    /\ state = "finishedSuccess"

(* The following formula holds true in the last state and violates the invariant *)
InvariantViolation ==
  state = "finishedSuccess" /\ Cardinality((DOMAIN fetchedLightBlocks)) = 3

================================================================================
(* Created by Apalache on Thu Jun 12 16:46:05 BRT 2025 *)
(* https://github.com/informalsystems/apalache *)

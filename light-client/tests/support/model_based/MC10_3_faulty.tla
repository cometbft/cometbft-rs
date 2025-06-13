------------------------- MODULE MC10_3_faulty ---------------------------

AllNodes == {"n1", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9", "n10"}
TRUSTED_HEIGHT == 1
TARGET_HEIGHT == 3
TRUSTING_PERIOD == 1400 \* two weeks, one day is 100 time units :-)
IS_PRIMARY_CORRECT == FALSE

VARIABLES
  \* @type: Str;
  state,
  \* @type: Int;
  nextHeight,
  \* @type: Int;
  nprobes,
  \* @type: Int -> [verified: BLOCK, current: BLOCK, now: Int, verdict: Str];
  history,
  \*@type: Int;
  now,
  \* @type: BLOCKCHAIN;
  blockchain,
  \* @type: Set(Str);
  Faulty,
  \* @type: Int -> BLOCK;
  fetchedLightBlocks,
  \* @type: BLOCKSTATUS;
  lightBlockStatus,
  \* @type: BLOCK;
  latestVerified,
  \* @type: BLOCKHEADER;
  prevVerified,
  \* @type: BLOCK;
  prevCurrent,
  \* @type: Int;
  prevNow,
  \* @type: Str;
  prevVerdict

INSTANCE LightTests

============================================================================

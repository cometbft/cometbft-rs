package generator

import (
	"time"

	"github.com/cometbft/cometbft/types"
)

const (
	SINGLE_STEP_SKIPPING_PATH = "./tests/json/single_step/skipping/"
)

// Validator Set

func caseSingleSkipOneBlock(valList ValList) {
	description := "Case: Trusted height=1, verifying signed header at height=3, should not expect error"

	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:3],
		valList.PrivVals[:3],
		1,
	)
	testCase := makeTestCase(description, initial, input, expectedOutputNoError)

	file := SINGLE_STEP_SKIPPING_PATH + "validator_set/skip_one_block.json"
	testCase.genJSON(file)
}

func caseSingleSkipFiveBlocks(valList ValList) {
	description := "Case: Trusted height=1, verifying signed header at height=7, should not expect error"

	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:3],
		valList.PrivVals[:3],
		5,
	)
	testCase := makeTestCase(description, initial, input, expectedOutputNoError)

	file := SINGLE_STEP_SKIPPING_PATH + "validator_set/skip_five_blocks.json"
	testCase.genJSON(file)
}

func caseSingleSkipValidatorSetChangesLessThanTrustLevel(valList ValList) {
	description := "Case: Trusted height=1 verifying signed header at height=7 while valset changes less than default trust level (1/3), should not expect error"

	copyValList := valList.Copy()
	initial, input, state, privVals := generateInitialAndInputSkipBlocks(
		copyValList.Validators[:4],
		copyValList.PrivVals[:4],
		3,
	)
	liteBlock, state, privVals := generateNextBlockWithNextValsUpdate(
		state,
		privVals,
		input[0].SignedHeader.Commit,
		copyValList.Validators[1:5],
		copyValList.PrivVals[1:5],
		thirdBlockTime.Add(30*time.Second),
	)
	liteBlock, state, _ = generateNextBlock(state, privVals, liteBlock.SignedHeader.Commit, thirdBlockTime.Add(35*time.Second))
	input[0] = liteBlock
	testCase := makeTestCase(description, initial, input, expectedOutputNoError)

	file := SINGLE_STEP_SKIPPING_PATH + "validator_set/valset_changes_less_than_trust_level.json"
	testCase.genJSON(file)
}

func caseSingleSkipValidatorSetChangesMoreThanTrustLevel(valList ValList) {
	description := "Case: Trusted height=1, verifying signed header at height=7 while valset changes more than default trust level (1/3), should expect error"

	copyValList := valList.Copy()
	initial, input, state, privVals := generateInitialAndInputSkipBlocks(
		copyValList.Validators[:4],
		copyValList.PrivVals[:4],
		3)
	liteBlock, state, privVals := generateNextBlockWithNextValsUpdate(
		state,
		privVals,
		input[0].SignedHeader.Commit,
		copyValList.Validators[3:7],
		copyValList.PrivVals[3:7],
		thirdBlockTime.Add(30*time.Second),
	)
	liteBlock, state, _ = generateNextBlock(state, privVals, liteBlock.SignedHeader.Commit, thirdBlockTime.Add(35*time.Second))
	input[0] = liteBlock
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SKIPPING_PATH + "validator_set/valset_changes_more_than_trust_level.json"
	testCase.genJSON(file)
}

// Commit

func caseSingleSkipCommitOneThirdValsDontSign(valList ValList) {
	description := "Case: Trusted height=1, verifying signed header at height=3 where 1/3 vals dont sign, should expect error"

	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:3],
		valList.PrivVals[:3],
		1,
	)
	input[0].SignedHeader.Commit.Signatures[0] = newAbsentCommitSig(
		input[0].SignedHeader.Commit.Signatures[0].ValidatorAddress,
	)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SKIPPING_PATH + "commit/one_third_vals_dont_sign.json"
	testCase.genJSON(file)
}

func caseSingleSkipCommitMoreThanTwoThirdsValsDidSign(valList ValList) {
	description := "Case: Trusted height=1, verifying signed header at height=3 where more than two-thirds vals did sign, should not expect error"

	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:4],
		valList.PrivVals[:4],
		1,
	)
	input[0].SignedHeader.Commit.Signatures[0] = newAbsentCommitSig(
		input[0].SignedHeader.Commit.Signatures[0].ValidatorAddress,
	)
	testCase := makeTestCase(description, initial, input, expectedOutputNoError)

	file := SINGLE_STEP_SKIPPING_PATH + "commit/more_than_two_third_vals_sign.json"
	testCase.genJSON(file)
}

func caseSingleSkipCommitNoSignatures(valList ValList) {
	description := "Case: one lite block, no signatures present in commit, expects error"
	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:1],
		valList.PrivVals[:1],
		5,
	)
	input[0].SignedHeader.Commit.Signatures = []types.CommitSig{}
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SKIPPING_PATH + "commit/no_signatures.json"
	testCase.genJSON(file)
}

func caseMoreSignaturesThanValidators(valList ValList) {
	description := "Case: commit contains more signatures than number of validators in the set, expects error"
	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:3],
		valList.PrivVals[:3],
		5,
	)
	last := len(input) - 1
	signatures := input[last].SignedHeader.Commit.Signatures
	extraSignature := types.CommitSig{
		BlockIDFlag:      types.BlockIDFlagCommit,
		Signature:        []byte(str64byte),
		ValidatorAddress: valList.Validators[4].Address,
		Timestamp:        signatures[0].Timestamp,
	}

	input[last].SignedHeader.Commit.Signatures = append(signatures, extraSignature)

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SKIPPING_PATH + "commit/more_signatures_than_validators.json"
	testCase.genJSON(file)
}

// Header

func caseSingleSkipHeaderOutOfTrustingPeriod(valList ValList) {
	description := "Case: Trusted height=1 but is out of trusting period, verifying signed header at height=6, expects an error"

	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:1],
		valList.PrivVals[:1],
		4,
	)
	initial.TrustingPeriod = 5 * time.Second

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SKIPPING_PATH + "header/out_of_trusting_period.json"
	testCase.genJSON(file)
}

func caseSingleSkipHeaderFromFuture(valList ValList) {
	description := "Case: Trusted height=1, fails at height=6 with error - header_from_future"

	initial, input, _, _ := generateInitialAndInputSkipBlocks(
		valList.Validators[:1],
		valList.PrivVals[:1],
		1,
	)

	initial.Now, _ = time.Parse(time.RFC3339, "2019-11-02T15:04:05Z")

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SKIPPING_PATH + "header/header_from_future.json"
	testCase.genJSON(file)
}

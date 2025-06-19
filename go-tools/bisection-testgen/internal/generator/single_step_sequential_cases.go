package generator

import (
	"time"

	"github.com/cometbft/cometbft/types"
)

const (
	str32byte            = "----This is a 32-byte string----"
	SINGLE_STEP_SEQ_PATH = "./tests/json/single_step/sequential/"
)

var (
	str64byte = []byte{206, 129, 9, 176, 142, 141, 188, 30, 197, 158, 80, 135, 172, 5, 239, 44, 219, 46, 60, 239, 9, 65, 151, 236, 221, 44, 72, 253, 191, 95, 20, 67, 175, 2, 133, 74, 3, 84, 20, 60, 142, 1, 0, 75, 129, 148, 2, 206, 180, 49, 223, 47, 41, 189, 149, 230, 247, 16, 48, 228, 39, 91, 154, 6}
)

// HEADER ----->

func caseSingleSeqHeaderWrongLastCommitHash(valList ValList) {

	description := "Case: one lite block, wrong last commit hash in header, with error"

	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Header.LastCommitHash = []byte("wrong hash")

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/wrong_last_commit_hash.json"
	testCase.genJSON(file)
}

func caseSingleSeqHeaderWrongLastBlockID(valList ValList) {

	description := "Case: one lite block, wrong last block ID in header, with error"

	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Header.LastBlockID.Hash = []byte("wrong hash")
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/wrong_last_block_id.json"
	testCase.genJSON(file)
}

func caseSingleSeqHeaderWrongChainID(valList ValList) {

	description := "Case: one lite block, wrong chain ID in header, with error"

	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Header.ChainID = "wrong-chain-id"
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/wrong_chain_id.json"
	testCase.genJSON(file)
}

func caseSingleSeqHeaderNonMonotonicHeight(valList ValList) {

	description := "Case: one lite block, non-monotonic height in header, with error"

	initial, input, _, privVals := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)

	// break header height
	input[0].SignedHeader.Header.Height = 1

	// update commit values for broken header height
	input[0].SignedHeader.Commit.BlockID.Hash = input[0].SignedHeader.Header.Hash()
	input[0].SignedHeader.Commit.Height = input[0].SignedHeader.Header.Height

	// re-sign the header with broken header time
	vote := input[0].SignedHeader.Commit.GetVote(0)
	privVals[0].SignVote(initial.SignedHeader.ChainID, vote.ToProto(), false)
	input[0].SignedHeader.Commit.Signatures[0].Signature = vote.Signature

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/non_monotonic_header_height.json"
	testCase.genJSON(file)
}

func caseSingleSeqHeaderWrongTimestamp(valList ValList) {

	description := "Case: one lite block, wrong timestamp in header, with error"

	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Header.Time = secondBlockTime.Add(1 * time.Minute)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/wrong_header_timestamp.json"
	testCase.genJSON(file)
}

func caseSingleSeqHeaderWrongValSetHash(valList ValList) {

	description := "Case: one lite block, wrong val set hash in header, with error"

	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Header.ValidatorsHash = []byte("wrong validator set hash")
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/wrong_valset_hash.json"
	testCase.genJSON(file)
}

func caseSingleSeqHeaderWrongNextValSetHash(valList ValList) {

	description := "Case: one lite block, wrong next val set hash in header, with error"

	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Header.NextValidatorsHash = []byte("wrong next validator set hash")
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/wrong_next_valset_hash.json"
	testCase.genJSON(file)
}

func caseNonMonotonicBftTime(valList ValList) {
	description := "Case: consecutive lite blocks with non-monotonic bft time, expects error"

	initial, input, _, privVals := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)

	// break bft time
	input[0].SignedHeader.Header.Time, _ = time.Parse(time.RFC3339, "2019-11-02T15:03:50Z")

	// update header hash in commit
	input[0].SignedHeader.Commit.BlockID.Hash = input[0].SignedHeader.Header.Hash()

	// re-sign the header with broken header time
	vote := input[0].SignedHeader.Commit.GetVote(0)
	privVals[0].SignVote(initial.SignedHeader.ChainID, vote.ToProto(), false)
	input[0].SignedHeader.Commit.Signatures[0].Signature = vote.Signature

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "header/non_monotonic_bft_time.json"
	testCase.genJSON(file)
}

// COMMIT ----->

func caseSingleSeqCommitWrongHeaderHash(valList ValList) {

	description := "Case: one lite block, wrong header hash, with error"
	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Commit.BlockID.Hash = []byte(str32byte)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "commit/wrong_header_hash.json"
	testCase.genJSON(file)
}

func caseSingleSeqCommitWrongHeight(valList ValList) {

	description := "Case: one lite block, wrong commit height, with error"
	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Commit.Height--
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "commit/wrong_commit_height.json"
	testCase.genJSON(file)
}

func caseSingleSeqCommitWrongVoteTimestamp(valList ValList) {

	description := "Case: one lite block, wrong vote timestamp, with error"
	initial, input, _, privVals := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)

	wrongTimestamp, _ := time.Parse(time.RFC3339, "2019-11-02T15:01:05Z")
	input[0].SignedHeader.Commit.Signatures[0].Timestamp = wrongTimestamp

	vote := input[0].SignedHeader.Commit.GetVote(0)
	privVals[0].SignVote(initial.SignedHeader.ChainID, vote.ToProto(), false)

	input[0].SignedHeader.Commit.Signatures[0].Signature = vote.Signature

	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "commit/wrong_vote_timestamp.json"
	testCase.genJSON(file)
}

func caseSingleSeqCommitWrongVoteSignature(valList ValList) {

	description := "Case: one lite block, wrong signature in vote, with error"
	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:1],
		valList.PrivVals[:1],
	)
	input[0].SignedHeader.Commit.Signatures[0].Signature = []byte(str64byte)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "commit/wrong_vote_signature.json"
	testCase.genJSON(file)
}

func caseSingleSeqCommitOneThirdValsDontSign(valList ValList) {

	description := "Case: one lite block, one-third vals don't sign, expects error"
	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:3],
		valList.PrivVals[:3],
	)
	input[0].SignedHeader.Commit.Signatures[0] = newAbsentCommitSig(
		input[0].SignedHeader.Commit.Signatures[0].ValidatorAddress,
	)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "commit/one_third_vals_dont_sign.json"
	testCase.genJSON(file)
}

func caseSingleSeqCommitMoreThanTwoThirdsValsDidSign(valList ValList) {

	description := "Case: one lite block, more than two-third vals did sign, no error"
	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:4],
		valList.PrivVals[:4],
	)
	input[0].SignedHeader.Commit.Signatures[0] = newAbsentCommitSig(
		input[0].SignedHeader.Commit.Signatures[0].ValidatorAddress,
	)
	testCase := makeTestCase(description, initial, input, expectedOutputNoError)

	file := SINGLE_STEP_SEQ_PATH + "commit/more_than_two_third_vals_sign.json"
	testCase.genJSON(file)
}
func caseSingleSeqCommitNilVote(valList ValList) {
	description := "Case: one lite block, less than one-third Nil votes, no error"
	initial, input, _, privVals := generateGeneralCase(
		valList.Validators[:4],
		valList.PrivVals[:4],
	)

	input[0].SignedHeader.Commit.Signatures[0].BlockIDFlag = types.BlockIDFlagNil

	vote := input[0].SignedHeader.Commit.GetVote(0)
	privVals[0].SignVote(initial.SignedHeader.ChainID, vote.ToProto(), false)

	input[0].SignedHeader.Commit.Signatures[0].Signature = vote.Signature

	testCase := makeTestCase(description, initial, input, expectedOutputNoError)

	file := SINGLE_STEP_SEQ_PATH + "commit/less_than_one_third_nil_votes.json"
	testCase.genJSON(file)
}

// VALIDATOR SET ---->

func caseSingleSeqValidatorSetOf1(valList ValList) {

	description := "Case: one lite block to fetch, one validator in the set, expects no error"
	testCase := generateAndMakeGeneralTestCase(
		description,
		valList.Validators[:1],
		valList.PrivVals[:1],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/1_validator.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetOf8(valList ValList) {

	description := "Case: one lite block to fetch, 8 validators in the set, expects no error"
	testCase := generateAndMakeGeneralTestCase(
		description,
		valList.Validators[:8],
		valList.PrivVals[:8],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/8_validators.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetOf128(valList ValList) {

	description := "Case: one lite block, 128 validators, no error"
	testCase := generateAndMakeGeneralTestCase(
		description,
		valList.Validators[:128],
		valList.PrivVals[:128],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/128_validators.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetEmpty(valList ValList) {

	description := "Case: one lite block, empty validator set, expects error"
	initial, input, _, _ := generateGeneralCase(
		valList.Validators[:2],
		valList.PrivVals[:2],
	)
	input[0].ValidatorSet = types.NewValidatorSet(nil)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "validator_set/empty_valset.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetAddTwiceVals(valList ValList) {

	description := "Case: two lite blocks, validator set increases 2x, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		valList.Validators[0:2],
		valList.PrivVals[0:2],
		valList.Validators[0:4],
		valList.PrivVals[0:4],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/valset_size_doubles.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetRemoveHalfVals(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, validator set reduces to half, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:4],
		copiedValList.PrivVals[0:4],
		copiedValList.Validators[0:2],
		copiedValList.PrivVals[0:2],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/valset_size_halves.json"
	testCase.genJSON(file)

}

func caseSingleSeqValidatorSetChangesOneThird(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, 1/3 validator set changes, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:3],
		copiedValList.PrivVals[0:3],
		copiedValList.Validators[1:4],
		copiedValList.PrivVals[1:4],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/one_third_valset_changes.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetChangesHalf(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, 1/2 validator set changes, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:4],
		copiedValList.PrivVals[0:4],
		copiedValList.Validators[2:6],
		copiedValList.PrivVals[2:6],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/half_valset_changes.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetChangesTwoThirds(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, 2/3 validator set changes, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:3],
		copiedValList.PrivVals[0:3],
		copiedValList.Validators[2:5],
		copiedValList.PrivVals[2:5],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/two_thirds_valset_changes.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetChangesFully(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, validator set changes completely, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:5],
		copiedValList.PrivVals[0:5],
		copiedValList.Validators[5:10],
		copiedValList.PrivVals[5:10],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/valset_changes_fully.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetChangesLessThanOneThird(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, less than 1/3 validator set changes, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:4],
		copiedValList.PrivVals[0:4],
		copiedValList.Validators[1:5],
		copiedValList.PrivVals[1:5],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/less_than_one_third_valset_changes.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetChangesMoreThanTwoThirds(valList ValList) {

	copiedValList := valList.Copy()
	description := "Case: two lite blocks, more than 2/3 validator set changes, no error"
	testCase := generateAndMakeNextValsUpdateTestCase(
		description,
		copiedValList.Validators[0:4],
		copiedValList.PrivVals[0:4],
		copiedValList.Validators[3:7],
		copiedValList.PrivVals[3:7],
		expectedOutputNoError,
	)
	file := SINGLE_STEP_SEQ_PATH + "validator_set/more_than_two_thirds_valset_changes.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetWrongValidatorSet(valList ValList) {

	var input []lightBlock
	description := "Case: one lite block, wrong validator set, expects error"

	signedHeader, state, _ := generateFirstBlock(
		valList.Validators[:3],
		valList.PrivVals[:3],
		firstBlockTime,
	)
	initial := generateInitial(signedHeader, state.NextValidators, TRUSTING_PERIOD, now)

	wrongVals := valList.Validators[3:6]
	wrongPrivVals := valList.PrivVals[3:6]
	wrongValSet := types.NewValidatorSet(wrongVals)
	state.Validators = wrongValSet
	state.NextValidators = wrongValSet

	liteBlock, state, _ := generateNextBlock(state, wrongPrivVals, initial.SignedHeader.Commit, secondBlockTime)
	input = append(input, liteBlock)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "validator_set/wrong_valset.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetFaultySigner(valList ValList) {

	copiedValList := valList.Copy()
	var input []lightBlock
	description := "Case: one lite block, faulty signer (not present in validator set), expects error"

	signedHeader, state, privVals := generateFirstBlock(
		copiedValList.Validators[:4],
		copiedValList.PrivVals[:4],
		firstBlockTime,
	)
	initial := generateInitial(signedHeader, state.NextValidators, TRUSTING_PERIOD, now)

	liteBlock, state, _ := generateNextBlock(state, privVals, initial.SignedHeader.Commit, secondBlockTime)

	liteBlock.ValidatorSet = types.NewValidatorSet(copiedValList.Validators[:3])
	liteBlock.SignedHeader.Header.ValidatorsHash = liteBlock.ValidatorSet.Hash()
	liteBlock.SignedHeader.Commit.BlockID.Hash = liteBlock.SignedHeader.Header.Hash()
	liteBlock.SignedHeader.Commit.Signatures = liteBlock.SignedHeader.Commit.Signatures[1:4]

	initial.NextValidatorSet = liteBlock.ValidatorSet

	input = append(input, liteBlock)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "validator_set/faulty_signer.json"
	testCase.genJSON(file)
}

func caseSingleSeqValidatorSetWrongValidatorPower(valList ValList) {

	copiedValList := valList.Copy()
	var input []lightBlock
	description := "Case: one lite block, changing a validator's power in validator set, expects error"

	signedHeader, state, privVals := generateFirstBlock(
		copiedValList.Validators[:3],
		copiedValList.PrivVals[:3],
		firstBlockTime,
	)
	initial := generateInitial(signedHeader, state.NextValidators, TRUSTING_PERIOD, now)

	state.Validators.Validators[0].VotingPower++
	state.NextValidators = state.Validators

	liteBlock, state, _ := generateNextBlock(state, privVals, initial.SignedHeader.Commit, secondBlockTime)
	input = append(input, liteBlock)
	testCase := makeTestCase(description, initial, input, expectedOutputError)

	file := SINGLE_STEP_SEQ_PATH + "validator_set/wrong_validator_power.json"
	testCase.genJSON(file)
}

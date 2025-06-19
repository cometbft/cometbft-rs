package generator

import (
	"time"

	"github.com/cometbft/cometbft/types"
)

const SINGLE_PEER_BISECTION_PATH = "./tests/json/bisection/single_peer/"

func caseBisectionHappyPath(valList ValList) {
	description := "Case: Trusted height=1, bisecting to verify height=11, should not expect error"
	valSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	testBisection, _, _ := generateGeneralBisectionCase(
		description,
		valSetChanges,
		int32(2),
	)

	file := SINGLE_PEER_BISECTION_PATH + "happy_path.json"
	testBisection.genJSON(file)

}

func caseBisectionWorstCase(valList ValList) {
	description := "Case: Trusted height=1, bisecting at every height, should not expect error"

	copiedValList := valList.Copy()
	valsArray := [][]*types.Validator{
		copiedValList.Validators[0:1],
		copiedValList.Validators[1:2],
		copiedValList.Validators[2:3],
		copiedValList.Validators[3:4],
		copiedValList.Validators[4:5],
		copiedValList.Validators[5:6],
		copiedValList.Validators[6:7],
		copiedValList.Validators[7:8],
		copiedValList.Validators[8:9],
		copiedValList.Validators[9:10],
		copiedValList.Validators[10:11],
	}
	privValsArray := []types.PrivValidatorsByAddress{
		copiedValList.PrivVals[0:1],
		copiedValList.PrivVals[1:2],
		copiedValList.PrivVals[2:3],
		copiedValList.PrivVals[3:4],
		copiedValList.PrivVals[4:5],
		copiedValList.PrivVals[5:6],
		copiedValList.PrivVals[6:7],
		copiedValList.PrivVals[7:8],
		copiedValList.PrivVals[8:9],
		copiedValList.PrivVals[9:10],
		copiedValList.PrivVals[10:11],
	}

	valSetChanges := ValSetChanges{}.makeValSetChanges(valsArray, privValsArray)

	testBisection, _, _ := generateGeneralBisectionCase(
		description,
		valSetChanges,
		10,
	)

	file := SINGLE_PEER_BISECTION_PATH + "worst_case.json"
	testBisection.genJSON(file)
}

func caseBisectionNotEnoughCommits(valList ValList) {
	description := "Case: Trusted height=1, fails at height 6 because more than one-third (trust level) vals didn't sign"

	valsArray := [][]*types.Validator{
		valList.Validators[:4],
		valList.Validators[:4],
		valList.Validators[:4],
		valList.Validators[:4],
		valList.Validators[:4],
		valList.Validators[:4],
	}

	privValsArray := []types.PrivValidatorsByAddress{
		valList.PrivVals[:4],
		valList.PrivVals[:4],
		valList.PrivVals[:4],
		valList.PrivVals[:4],
		valList.PrivVals[:4],
		valList.PrivVals[:3],
	}

	valSetChanges := ValSetChanges{}.makeValSetChanges(valsArray, privValsArray)
	testBisection, _, _ := generateGeneralBisectionCase(
		description,
		valSetChanges,
		3,
	)

	last := len(testBisection.Primary.LightBlocks) - 1
	testBisection.Primary.LightBlocks[last].SignedHeader.Commit.Signatures[0] = newAbsentCommitSig(
		testBisection.Primary.LightBlocks[last].SignedHeader.Commit.Signatures[0].ValidatorAddress,
	)
	testBisection.Primary.LightBlocks[last].SignedHeader.Commit.Signatures[1] = newAbsentCommitSig(
		testBisection.Primary.LightBlocks[last].SignedHeader.Commit.Signatures[1].ValidatorAddress,
	)

	testBisection.ExpectedOutput = expectedOutputError

	file := SINGLE_PEER_BISECTION_PATH + "not_enough_commits.json"
	testBisection.genJSON(file)

}

func caseBisectionHeaderOutOfTrustingPeriod(valList ValList) {
	description := "Case: Trusted height=1, fails at height 11 because header at height 1 runs out of trusting period while bisecting"
	valSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	testBisection, _, _ := generateGeneralBisectionCase(
		description,
		valSetChanges,
		int32(1),
	)

	testBisection.TrustOptions.Period = 30 * time.Second
	testBisection.ExpectedOutput = expectedOutputError

	file := SINGLE_PEER_BISECTION_PATH + "header_out_of_trusting_period.json"
	testBisection.genJSON(file)
}

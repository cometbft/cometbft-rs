package generator

// GenerateSingleStepSequentialCases creates three json files each for validator set, commit and header cases
// These cases are categorized according to the data structure it is trying to test (e.g. Validator set, Commit, etc...)
// It produces cases that test the single step sequential verification
// what this means is, given a trusted state and height can the lite node verify the next block height?
func GenerateSingleStepSequentialCases(jsonValList string) {

	valList := GetValList(jsonValList)

	// ValidatorSet
	caseSingleSeqValidatorSetOf1(valList)
	caseSingleSeqValidatorSetOf8(valList)
	caseSingleSeqValidatorSetOf128(valList)

	caseSingleSeqValidatorSetAddTwiceVals(valList)
	caseSingleSeqValidatorSetRemoveHalfVals(valList)

	caseSingleSeqValidatorSetChangesOneThird(valList)
	caseSingleSeqValidatorSetChangesHalf(valList)
	caseSingleSeqValidatorSetChangesTwoThirds(valList)
	caseSingleSeqValidatorSetChangesFully(valList)
	caseSingleSeqValidatorSetChangesLessThanOneThird(valList)
	caseSingleSeqValidatorSetChangesMoreThanTwoThirds(valList)
	caseSingleSeqValidatorSetWrongValidatorSet(valList)
	caseSingleSeqValidatorSetFaultySigner(valList)
	// caseSingleSeqValidatorSetWrongValidatorPower(valList)

	// Commit
	caseSingleSeqCommitWrongHeaderHash(valList)
	caseSingleSeqCommitWrongHeight(valList)
	caseSingleSeqCommitWrongVoteSignature(valList)

	caseSingleSeqCommitOneThirdValsDontSign(valList)         // error
	caseSingleSeqCommitMoreThanTwoThirdsValsDidSign(valList) // not an error

	caseSingleSeqCommitNilVote(valList)

	// Header
	caseSingleSeqHeaderWrongLastCommitHash(valList)
	caseSingleSeqHeaderWrongLastBlockID(valList)
	caseSingleSeqHeaderWrongChainID(valList)
	caseSingleSeqHeaderNonMonotonicHeight(valList)
	caseSingleSeqHeaderWrongTimestamp(valList)
	caseSingleSeqHeaderWrongValSetHash(valList)
	caseSingleSeqHeaderWrongNextValSetHash(valList)
	caseNonMonotonicBftTime(valList)

}

// GenerateSingleStepSkippingCases creates three json files each for validator set, commit and header cases
// These cases test the single step skipping verification
// which means, given a trusted height and state can the lite node jump to a certain block height?
func GenerateSingleStepSkippingCases(jsonValList string) {

	valList := GetValList(jsonValList)

	// ValidatorSet
	caseSingleSkipOneBlock(valList)
	caseSingleSkipFiveBlocks(valList)
	caseSingleSkipValidatorSetChangesLessThanTrustLevel(valList)
	caseSingleSkipValidatorSetChangesMoreThanTrustLevel(valList)

	// Commit

	caseSingleSkipCommitOneThirdValsDontSign(valList)         // error
	caseSingleSkipCommitMoreThanTwoThirdsValsDidSign(valList) // not an error
	caseSingleSkipCommitNoSignatures(valList)
	caseMoreSignaturesThanValidators(valList)

	// Header

	caseSingleSkipHeaderOutOfTrustingPeriod(valList)
	caseSingleSkipHeaderFromFuture(valList)
}

func GenerateManyHeaderBisectionCases(jsonValList string) {

	valList := GetValList(jsonValList)

	// Single-peer
	caseBisectionHappyPath(valList)
	caseBisectionWorstCase(valList)
	caseBisectionMaliciousValidatorSet(valList)
	caseBisectionNotEnoughCommits(valList)
	caseBisectionHeaderOutOfTrustingPeriod(valList)

	// Multi-peer
	caseBisectionConflictingValidCommitsFromTheOnlyWitness(valList)
	caseBisectionConflictingValidCommitsFromOneOfTheWitnesses(valList)
	caseBisectionConflictingHeaders(valList)
}

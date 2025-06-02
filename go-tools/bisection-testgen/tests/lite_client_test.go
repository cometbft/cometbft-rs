package tests

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"

	cmtjson "github.com/cometbft/cometbft/libs/json"
	lite "github.com/cometbft/cometbft/light"
	"github.com/cometbft/cometbft-rs/go-tools/bisection-testgen/internal/generator"

	dbm "github.com/cometbft/cometbft-db"
	"github.com/cometbft/cometbft/light/provider"
	dbs "github.com/cometbft/cometbft/light/store/db"
)

func TestVerify(t *testing.T) {

	tests, err := getTestPaths("./json/single_step/")
	if err != nil {
		fmt.Println(err)
	}

	for _, test := range tests {
		data := generator.ReadFile(test)

		var testCase generator.TestCase
		err := cmtjson.Unmarshal(data, &testCase)
		if err != nil {
			fmt.Printf("error: %v", err)
		}

		// chainID := testCase.Initial.SignedHeader.Header.ChainID
		trustedSignedHeader := testCase.Initial.SignedHeader
		trustedNextVals := testCase.Initial.NextValidatorSet
		trustingPeriod := testCase.Initial.TrustingPeriod
		now := testCase.Initial.Now
		trustLevel := lite.DefaultTrustLevel
		expectedOutput := testCase.ExpectedOutput
		expectsError := expectedOutput == "error"

		for _, input := range testCase.Input {

			newSignedHeader := input.SignedHeader
			newVals := input.ValidatorSet

			e := lite.Verify(
				trustedSignedHeader,
				trustedNextVals,
				newSignedHeader,
				newVals,
				trustingPeriod,
				now,
				time.Second,
				trustLevel,
			)
			err := e != nil
			fmt.Printf("\n%s, \nError: %v \n", testCase.Description, e)
			if (err && !expectsError) || (!err && expectsError) {
				t.Errorf("\n Failing test: %s \n Error: %v \n Expected error: %v", testCase.Description, e, expectedOutput)

			} else {
				trustedSignedHeader = newSignedHeader
				trustedNextVals = input.NextValidatorSet
			}
		}
	}

}

func TestBisection(t *testing.T) {
	tests, err := getTestPaths("./json/bisection/")
	if err != nil {
		fmt.Println(err)
	}

	for _, test := range tests {
		// we skip this one for now because the current version (v0.33.6)
		// does not panic on receiving conflicting commits from witnesses
		skippedTest := "json/bisection/multi_peer/conflicting_valid_commits_from_one_of_the_witnesses.json"
		if test == skippedTest {
			fmt.Printf("\ntest case skipped: %v", skippedTest)
			continue
		}

		data := generator.ReadFile(test)

		var testBisection generator.TestBisection
		e := cmtjson.Unmarshal(data, &testBisection)
		if e != nil {
			fmt.Printf("error: %v", e)
		}

		fmt.Println(testBisection.Description)

		trustedStore := dbs.New(dbm.NewMemDB(), testBisection.Primary.ChainID())
		witnesses := testBisection.Witnesses
		trustOptions := lite.TrustOptions{
			Period: testBisection.TrustOptions.Period,
			Height: testBisection.TrustOptions.Height,
			Hash:   testBisection.TrustOptions.Hash,
		}
		trustLevel := testBisection.TrustOptions.TrustLevel
		expectedOutput := testBisection.ExpectedOutput
		witnessesAsProviders := make([]provider.Provider, 0, len(witnesses))
		for _, w := range witnesses {
			witnessesAsProviders = append(witnessesAsProviders, w)
		}

		client, e := lite.NewClient(
			context.Background(),
			testBisection.Primary.ChainID(),
			trustOptions,
			testBisection.Primary,
			witnessesAsProviders,
			trustedStore,
			lite.SkippingVerification(trustLevel))
		if e != nil {
			fmt.Println(e)
		}

		height := testBisection.HeightToVerify
		_, e = client.VerifyLightBlockAtHeight(context.Background(), height, testBisection.Now)
		// ---
		fmt.Println(e)
		// ---
		err := e != nil
		expectsError := expectedOutput == "error"
		if (err && !expectsError) || (!err && expectsError) {
			t.Errorf("\n Failing test: %s \n Error: %v \n Expected error: %v", testBisection.Description, e, testBisection.ExpectedOutput)

		}
	}
}

func getTestPaths(folder string) ([]string, error) {
	var tests []string
	err := filepath.Walk(folder, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			fmt.Printf("prevent panic by handling failure accessing a path %q: %v\n", path, err)
			return err
		}
		if !info.IsDir() {
			tests = append(tests, path)
		}
		return nil
	})
	if err != nil {
		fmt.Printf("error: %v", err)
		return nil, err
	}
	return tests, nil
}

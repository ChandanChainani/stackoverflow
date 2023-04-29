package main

import (
	"fmt"
	"go.dedis.ch/kyber/v3"
	"go.dedis.ch/kyber/v3/suites"
	"log"

	"bytes"
)

func main() {
	// suite := suites.MustFind("Ed25519")
	// // fmt.Println(kyber.Scalar())
	// r := suite.Scalar()
	// r.Mul(x, c_scl).Sub(v, r)

	// r_by, err := r.MarshalBinary()

	// // _, err = connection.Write(r_by)
	// // defer connection.Close()
	// // buffer5 := make([]byte, 1024)
	// // mLen5, err := connection.Read(buffer5)
	// fmt.Println(r_by)
	// buffer5 := r_by
	// if err != nil {
	//     fmt.Println("Error reading:", err.Error())
	// }
	// r := buffer5[:mLen5]

	// rG := suite.Point().Mul(r, G_pt)

	suite := suites.MustFind("Ed25519")            // Use the edwards25519-curve
	a := suite.Scalar().Pick(suite.RandomStream()) // Alice's private key
	fmt.Printf("a: %#v\n", a)
	a_by, err := a.MarshalBinary()
	if err != nil {
		log.Fatal("Something went wrong")
	}
	fmt.Printf("bytes: %#v\n", a_by)
	A := suite.Point().Mul(a, nil)
	fmt.Printf("A: %#v\n", A)
	b := suite.Scalar()
	b.SetBytes(a_by)
	fmt.Printf("b: %#v\n", b)
	fmt.Println()

	buf := bytes.Buffer{}
	suite.Write(&buf, &a)
	fmt.Printf("buf W: %#v\n", buf)
	var c kyber.Scalar
	bufBytes := buf.Bytes()
	fmt.Println(bufBytes)
	if err := suite.Read(bytes.NewBuffer(bufBytes), &c); err != nil {
		log.Fatal("Something went wrong")
	}
	fmt.Printf("buf R: %#v\n", c)
}

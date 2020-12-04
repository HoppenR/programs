package main

type TweakInterface interface {
	Validate() bool
}

type override struct {
	TweakInterface
}

func (cid override) Validate() bool {
	return cid.TweakInterface.Validate() || true
}

func Tweak(data TweakInterface) TweakInterface {
	return &override{data}
}


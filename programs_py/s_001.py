# s_001
# Built with Seahorse v0.2.0

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class Hello(Account):
  bump: u8

@instruction
def init(owner: Signer, hello: Empty[Hello]):
    bump = hello.bump()
    hello = hello.init(
        payer=owner,
        seeds=['hello']
    )
    hello.bump = bump

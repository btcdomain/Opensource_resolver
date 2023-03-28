%builtins output range_check bitwise

from starkware.cairo.common.serialize import serialize_word
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.uint256 import Uint256
from starkware.cairo.common.cairo_secp.constants import N0, N1, N2, BASE
from starkware.cairo.common.cairo_secp.ec import EcPoint, ec_add, ec_mul
from starkware.cairo.common.cairo_secp.signature import div_mod_n, validate_signature_entry, get_generator_point
from starkware.cairo.common.cairo_secp.bigint import uint256_to_bigint, BigInt3
from starkware.cairo.common.cairo_builtins import BitwiseBuiltin

func main{output_ptr: felt*, range_check_ptr, bitwise_ptr: BitwiseBuiltin*}() {
    alloc_locals;

    local msg_hash: Uint256;
    local r: Uint256;
    local s: Uint256;

    %{
        import hashlib

        signature = program_input["signature"]
        sign_bytes = bytes.fromhex(signature)

        r = int.from_bytes(sign_bytes[5:37], 'big')
        s = int.from_bytes(sign_bytes[39:], 'big')
        ids.r.low = r & ((1 << 128) - 1)
        ids.r.high = r >> 128
        ids.s.low = s & ((1 << 128) - 1)
        ids.s.high = s >> 128

        msg_bytes = program_input["data"]
        msg_bytes = bytes(msg_bytes)
        msg_hash = hashlib.sha256(msg_bytes).digest()
        msg_hash_int = int.from_bytes(msg_hash, 'big')
        ids.msg_hash.low = msg_hash_int & ((1 << 128) - 1)
        ids.msg_hash.high = msg_hash_int >> 128
    %}

    let (msg_hash_bigint: BigInt3) = uint256_to_bigint(msg_hash);
    let (r_bigint: BigInt3) = uint256_to_bigint(r);
    let (s_bigint: BigInt3) = uint256_to_bigint(s);
    let (public_key_pt) = public_key_point();
    verify_ecdsa(public_key_pt=public_key_pt, msg_hash=msg_hash_bigint,r=r_bigint, s=s_bigint);
    return ();
}

func verify_ecdsa{range_check_ptr}(
        public_key_pt : EcPoint, msg_hash : BigInt3, r : BigInt3, s : BigInt3) {
    alloc_locals;

    validate_signature_entry(r);
    validate_signature_entry(s);

    let gen_pt = get_generator_point();

    // # Compute u1 and u2.
    let (u1 : BigInt3) = div_mod_n(msg_hash, s);
    let (u2 : BigInt3) = div_mod_n(r, s);

    let (gen_u1) = ec_mul(gen_pt.point, u1);
    let (pub_u2) = ec_mul(public_key_pt, u2);
    let (res) = ec_add(gen_u1, pub_u2);

    assert res.x = r;
    return ();
}

func public_key_point() -> (point: EcPoint) {
    return (point = EcPoint(
        BigInt3(21400756543238744274183860, 52255194649186086387900667, 6716313820872772989164962),
        BigInt3(25796246552686708830209044, 17146282563730156014111922, 18665702648792584675825455)
    ));
}
use anchor_lang::prelude::*;
use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY_SOMBRERO: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 3,
    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        20, 204, 21, 201, 167, 84, 81, 155, 108, 247, 239, 251, 182, 147, 96, 30, 51, 197, 115, 27,
        159, 62, 19, 190, 9, 54, 12, 156, 26, 69, 134, 222, 45, 97, 100, 147, 211, 180, 246, 182,
        41, 140, 240, 112, 49, 203, 227, 198, 104, 243, 39, 236, 101, 17, 7, 248, 212, 152, 97, 11,
        25, 20, 102, 102, 22, 172, 67, 229, 115, 128, 77, 75, 197, 191, 145, 232, 136, 210, 26,
        243, 226, 230, 228, 19, 41, 233, 144, 130, 34, 153, 92, 98, 190, 170, 170, 223, 24, 184,
        43, 55, 134, 224, 48, 127, 99, 226, 240, 210, 206, 64, 159, 255, 27, 67, 225, 69, 131, 144,
        201, 73, 137, 10, 107, 15, 183, 148, 65, 101,
    ],

    vk_ic: &[
        [
            10, 179, 5, 22, 243, 244, 18, 24, 48, 121, 147, 146, 200, 18, 101, 234, 226, 72, 20,
            59, 144, 142, 142, 200, 216, 214, 253, 231, 188, 166, 144, 171, 3, 203, 125, 111, 22,
            137, 152, 161, 154, 21, 103, 235, 78, 9, 202, 74, 144, 149, 66, 101, 89, 148, 105, 147,
            161, 243, 31, 191, 49, 80, 90, 1,
        ],
        [
            24, 184, 190, 192, 208, 91, 169, 114, 114, 53, 132, 184, 218, 186, 143, 140, 159, 82,
            158, 183, 62, 123, 74, 30, 81, 197, 12, 135, 159, 20, 242, 162, 6, 46, 151, 47, 96,
            200, 38, 125, 215, 101, 52, 32, 66, 200, 251, 148, 212, 80, 235, 211, 213, 154, 172,
            10, 11, 113, 55, 147, 119, 81, 245, 129,
        ],
        [
            33, 125, 5, 137, 252, 17, 237, 20, 191, 63, 173, 30, 39, 34, 94, 253, 156, 19, 120, 40,
            48, 205, 195, 97, 245, 233, 113, 224, 234, 50, 69, 126, 23, 89, 100, 113, 177, 208,
            196, 26, 117, 135, 45, 20, 244, 242, 146, 135, 215, 65, 192, 13, 45, 0, 2, 155, 90, 65,
            226, 122, 228, 191, 164, 95,
        ],
        [
            12, 61, 138, 203, 157, 82, 83, 234, 255, 114, 214, 224, 70, 99, 243, 140, 214, 49, 90,
            177, 184, 201, 174, 153, 88, 72, 187, 228, 161, 61, 185, 21, 30, 110, 93, 21, 116, 21,
            137, 33, 247, 17, 210, 196, 107, 172, 219, 20, 216, 171, 154, 58, 72, 212, 19, 209,
            191, 126, 237, 173, 61, 24, 53, 27,
        ],
    ],
};
#[account]
pub struct ZKsombreroProofInputs {
    public_app_verifier: u8,
    transaction_hash: u8,
    public_z: u8,
    is_app_in_utxo: [[u8; 4]; 1],
    tx_integrity_hash: u8,
    in_amount: [[u8; 2]; 4],
    in_public_key: [u8; 4],
    in_blinding: [u8; 4],
    in_app_data_hash: [u8; 4],
    in_pool_type: [u8; 4],
    in_verifier_pubkey: [u8; 4],
    in_indices: [[[u8; 3]; 2]; 4],
    output_commitment: [u8; 4],
    out_amount: [[u8; 2]; 4],
    out_pubkey: [u8; 4],
    out_blinding: [u8; 4],
    out_app_data_hash: [u8; 4],
    out_indices: [[[u8; 3]; 2]; 4],
    out_pool_type: [u8; 4],
    out_verifier_pubkey: [u8; 4],
    asset_pubkeys: [u8; 3],
    transaction_version: u8,
    x: u8,
    y: u8,
}
#[account]
pub struct ZKsombreroPublicInputs {
    public_app_verifier: u8,
    transaction_hash: u8,
    public_z: u8,
}
#[account]
pub struct InstructionDataLightInstructionSombreroSecond {
    public_app_verifier: [u8; 32],
    transaction_hash: [u8; 32],
    public_z: [u8; 32],
}

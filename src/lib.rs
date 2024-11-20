// Copyright 2021 Gregory Chadwick <mail@gregchadwick.co.uk>
// Licensed under the Apache License Version 2.0, with LLVM Exceptions, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! RISC-V instruction set simulator library
//!
//! Containts the building blocks for a RISC-V ISS. The seperate rrs-cli uses rrs-lib to implement
//! a CLI driven ISS.

pub mod instruction_formats;
pub mod process_instruction;

use downcast_rs::{impl_downcast, Downcast};

pub use process_instruction::process_instruction;
pub use process_instruction::process_instruction_128;

/// A trait for objects which do something with RISC-V instructions (e.g. execute them or print a
/// disassembly string).
///
/// There is one function per RISC-V instruction. Each function takes the appropriate struct from
/// [instruction_formats] giving access to the decoded fields of the instruction. All functions
/// return the [InstructionProcessor::InstructionResult] associated type.
pub trait InstructionProcessor {
    type InstructionResult;

    fn process_add128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sub128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sll128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_slt128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sltu128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_xor128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_srl128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sra128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_or128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_and128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;

    fn process_addi128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_adjsp(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_stackaddr(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_slli128(
        &mut self,
        dec_insn: instruction_formats::ITypeShamt,
    ) -> Self::InstructionResult;
    fn process_slti128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_sltui128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_xori128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_srli128(
        &mut self,
        dec_insn: instruction_formats::ITypeShamt,
    ) -> Self::InstructionResult;
    fn process_srai128(
        &mut self,
        dec_insn: instruction_formats::ITypeShamt,
    ) -> Self::InstructionResult;
    fn process_ori128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_andi128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_lui128(&mut self, dec_insn: instruction_formats::UType) -> Self::InstructionResult;
    fn process_auipc128(&mut self, dec_insn: instruction_formats::UType) -> Self::InstructionResult;

    fn process_beq128(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bne128(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_blt128(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bltu128(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bge128(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bgeu128(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;

    fn process_lb128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lbu128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lh128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lhu128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lw128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_sb128(&mut self, dec_insn: instruction_formats::SType) -> Self::InstructionResult;
    fn process_sh128(&mut self, dec_insn: instruction_formats::SType) -> Self::InstructionResult;
    fn process_sw128(&mut self, dec_insn: instruction_formats::SType) -> Self::InstructionResult;

    fn process_jal128(&mut self, dec_insn: instruction_formats::JType) -> Self::InstructionResult;
    fn process_jalr128(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_mul128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_mulh128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_mulhu128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_mulhsu128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;

    fn process_div128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_divu128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_rem128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_remu128(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;


    fn process_add(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sub(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sll(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_slt(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sltu(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_xor(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_srl(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_sra(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_or(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_and(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;

    fn process_addi(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_slli(
        &mut self,
        dec_insn: instruction_formats::ITypeShamt,
    ) -> Self::InstructionResult;
    fn process_slti(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_sltui(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_xori(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_srli(
        &mut self,
        dec_insn: instruction_formats::ITypeShamt,
    ) -> Self::InstructionResult;
    fn process_srai(
        &mut self,
        dec_insn: instruction_formats::ITypeShamt,
    ) -> Self::InstructionResult;
    fn process_ori(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_andi(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_lui(&mut self, dec_insn: instruction_formats::UType) -> Self::InstructionResult;
    fn process_auipc(&mut self, dec_insn: instruction_formats::UType) -> Self::InstructionResult;

    fn process_beq(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bne(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_blt(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bltu(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bge(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;
    fn process_bgeu(&mut self, dec_insn: instruction_formats::BType) -> Self::InstructionResult;

    fn process_lb(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lbu(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lh(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lhu(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;
    fn process_lw(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_sb(&mut self, dec_insn: instruction_formats::SType) -> Self::InstructionResult;
    fn process_sh(&mut self, dec_insn: instruction_formats::SType) -> Self::InstructionResult;
    fn process_sw(&mut self, dec_insn: instruction_formats::SType) -> Self::InstructionResult;

    fn process_jal(&mut self, dec_insn: instruction_formats::JType) -> Self::InstructionResult;
    fn process_jalr(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_mul(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_mulh(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_mulhu(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_mulhsu(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;

    fn process_div(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_divu(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_rem(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;
    fn process_remu(&mut self, dec_insn: instruction_formats::RType) -> Self::InstructionResult;

    fn process_fence(&mut self, dec_insn: instruction_formats::IType) -> Self::InstructionResult;

    fn process_csrrw(&mut self, dec_insn: instruction_formats::ITypeCSR)
        -> Self::InstructionResult;
    fn process_csrrs(&mut self, dec_insn: instruction_formats::ITypeCSR)
        -> Self::InstructionResult;
    fn process_csrrc(&mut self, dec_insn: instruction_formats::ITypeCSR)
        -> Self::InstructionResult;
    fn process_csrrwi(
        &mut self,
        dec_insn: instruction_formats::ITypeCSR,
    ) -> Self::InstructionResult;
    fn process_csrrsi(
        &mut self,
        dec_insn: instruction_formats::ITypeCSR,
    ) -> Self::InstructionResult;
    fn process_csrrci(
        &mut self,
        dec_insn: instruction_formats::ITypeCSR,
    ) -> Self::InstructionResult;

    fn process_ecall(&mut self) -> Self::InstructionResult;
    fn process_ebreak(&mut self) -> Self::InstructionResult;
    fn process_wfi(&mut self) -> Self::InstructionResult;
    fn process_mret(&mut self) -> Self::InstructionResult;
}

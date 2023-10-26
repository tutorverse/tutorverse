import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tutorverse } from "../target/types/tutorverse";

const textEncoder = new TextEncoder();

export const deriveConfig = (program: Program<Tutorverse>) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("config")],
        program.programId
    );

export const deriveTeacherProfile = (program: Program<Tutorverse>, payer: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("teacher"), payer.toBuffer()],
        program.programId
    );

export const deriveTeacherById = (program: Program<Tutorverse>, id: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("teacher_by_id"), new anchor.BN(id).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveStudentProfile = (program: Program<Tutorverse>, payer: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("student"), payer.toBuffer()],
        program.programId
    );

export const deriveStudentById = (program: Program<Tutorverse>, id: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("student_by_id"), new anchor.BN(id).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveSubjectConfig = (program: Program<Tutorverse>, subjectId: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("subject_config"), new anchor.BN(subjectId).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveSubjectToTeacher = (program: Program<Tutorverse>, subjectId: number, teacherNum: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("subject_teacher"), new anchor.BN(subjectId).toArrayLike(Buffer, "le", 4), new anchor.BN(teacherNum).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveLesson = (program: Program<Tutorverse>, teacherProfile: anchor.web3.PublicKey, teacherLessonNum: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("lesson"), teacherProfile.toBuffer(), new anchor.BN(teacherLessonNum).toArrayLike(Buffer, "le", 4)],
        program.programId
    );
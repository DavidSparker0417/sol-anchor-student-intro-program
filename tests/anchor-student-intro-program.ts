import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntroProgram } from "../target/types/anchor_student_intro_program";
import { expect } from "chai";

describe("anchor-student-intro-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.AnchorStudentIntroProgram as Program<AnchorStudentIntroProgram>;

  const student = {
    name: "Braziski",
    message: "Enthusiast in Blockchain bot."
  }

  const [studentPda] = anchor.web3.PublicKey
    .findProgramAddressSync([provider.wallet.publicKey.toBuffer()], program.programId)

  it("1.Add introduction", async () => {
    // Add your test here.
    const tx = await program.methods
      .addStudentIntro(student.name, student.message)
      .rpc();

    const studentAccount = await program.account.studentInfoState.fetch(studentPda);
    expect(studentAccount.name).to.equal(student.name);
    expect(studentAccount.message).to.equal(student.message);
  });

  it('2.Add introduction twice', async() => {
    let transacion_success = false
    try {
      const tx = await program.methods
        .addStudentIntro("Alice", "My name is Alice.")
        .rpc() 
      transacion_success = true
    } catch (error) {
      transacion_success = false
    }
    expect(transacion_success, 'transaction should be failed').to.equal(false)
  })

  it('3. Update student intro', async() => {
    const newMessage = "Remote computer broker!"
    const tx = await program.methods
      .updateStudent(student.name, newMessage)
      .rpc()
    const studentAccount = await program.account.studentInfoState.fetch(studentPda)
    expect(studentAccount.name).to.equal(student.name)
    expect(studentAccount.message).to.equal(newMessage)
  })
});

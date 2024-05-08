use anchor_lang::prelude::*;

declare_id!("HaZ6i8ytt2QxDdB3gZTriqYBF2JJSj1xn3V2X2vNn95J");

#[program]
pub mod anchor_student_intro_program {
    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudent>,
        name: String,
        message: String
    ) -> Result<()> {
        let introducer = &mut ctx.accounts.student_introducer;
        msg!("Adding student info");
        msg!("Name: {}", name);
        msg!("Message: {}", message);
        introducer.name = name;
        introducer.message = message;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct AddStudent<'info> {
    #[account(
        init, 
        seeds = [initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + (4 + name.len()) + (4 + message.len())
        )
    ]
    student_introducer: Account<'info, StudentInfoState>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>
}

#[account]
pub struct StudentInfoState {
    pub introducer: Pubkey, // 32
    pub name: String,       // 4 + len
    pub message: String     // 4 + len
}
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
        let introducer = &mut ctx.accounts.student_account;
        msg!("Adding student info");
        msg!("Name: {}", name);
        msg!("Message: {}", message);
        introducer.name = name;
        introducer.message = message;
        Ok(())
    }

    pub fn update_student(
        ctx: Context<UpdateStudent>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("Updating student intorduction.");
        msg!("Name {}", name);
        msg!("Message {}", message);
        let student_account = &mut ctx.accounts.student_account;
        student_account.message = message; 
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
    student_account: Account<'info, StudentInfoState>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        seeds = [initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + (4 + name.len()) + (4 + message.len()),
        realloc::zero = true,
        realloc::payer = initializer,
    )]
    student_account: Account<'info, StudentInfoState>,
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
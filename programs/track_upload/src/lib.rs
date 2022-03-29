use anchor_lang::prelude::*;

declare_id!("6yT4AFpityTwxz7AdVrSJTckGyPyrXL8MKuutmBbDTnj");

#[program]
pub mod track_upload {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        cid: String,
        artist: Option<String>,
        title: Option<String>,
    ) -> Result<()> {
        let track = &mut ctx.accounts.track;
        // Validate Lengths
        require!(cid.chars().count() <= 60, TrackError::InvalidCID);
        track.signer = ctx.accounts.signer.key();
        track.cid = cid;
        if let Some(a) = artist {
            require!(a.chars().count() <= 32, TrackError::TrackTooLong);
            track.artist = a;
        };
        if let Some(t) = title {
                require!(t.chars().count() <= 32, TrackError::ArtistTooLong);
                track.track_title = t;
            };

        emit!(TrackEvent::new(
            &track.cid,
            &track.artist,
            &track.track_title,
            &track.signer,
            &track.key()
        ));
        Ok(())
    }

    // Would be prefered to take Option<String> as args but
    // running into issues with serialization
    pub fn update(
        ctx: Context<UpdateTrack>,
        cid: Option<String>,
        artist: Option<String>,
        title: Option<String>,
    ) -> Result<()> {
        let track = &mut ctx.accounts.track;
        require!(
            ctx.accounts.signer.key() == track.signer,
            TrackError::UnauthorizedUser
        );
        // Validate Lengths
        if let Some(c) = cid {
            require!(c.chars().count() <= 47, TrackError::InvalidCID);
            track.cid = c;
        }
        if let Some(a) = artist {
            require!(a.chars().count() <= 32, TrackError::TrackTooLong);
            track.artist = a;
        }
        if let Some(t) = title {
            require!(t.chars().count() <= 32, TrackError::ArtistTooLong);            
            track.track_title = t;
        }
        emit!(TrackEvent::new(
            &track.cid,
            &track.artist,
            &track.track_title,
            &track.signer,
            &track.key()
        ));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 174)]
    pub track: Account<'info, Track>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTrack<'info> {
    #[account(mut, has_one = signer)]
    pub track: Account<'info, Track>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Track {
    pub signer: Pubkey,      //64
    pub cid: String,         //47
    pub artist: String,      //32
    pub track_title: String, //32
}

#[event]
pub struct TrackEvent {
    pub cid: String,
    pub artist: String,
    pub track_title: String,
    pub signer: Pubkey,
    pub track: Pubkey,
}

impl TrackEvent {
    pub fn new(
        cid: &str, 
        artist: &str, 
        title: &str,
        signer: &Pubkey,
        track: &Pubkey
    ) -> TrackEvent {
        TrackEvent {
            cid: cid.to_string(),
            artist: artist.to_string(),
            track_title: title.to_string(),
            signer: *signer,
            track: *track
        }
    }
}

#[error_code]
pub enum TrackError {
    #[msg("Only the original signer can update tracks.")]
    UnauthorizedUser,
    #[msg("Track should be up to 32 characters")]
    TrackTooLong,
    #[msg("Artist should be up to 32 characters")]
    ArtistTooLong,
    #[msg("CID is not valid")]
    InvalidCID,
}

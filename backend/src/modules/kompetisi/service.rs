use actix_web::error::ErrorBadRequest;
use diesel::prelude::*;

use crate::UnwrappedPool;

use super::{
    dto::{create::CreateKompetisi, propose_kompetisi::ProposeKompetisiInput},
    model::Kompetisi,
};

pub fn create_kompetisi<'a>(
    connection: &UnwrappedPool,
    kompetisi_data: CreateKompetisi,
) -> Result<Kompetisi, diesel::result::Error> {
    use crate::schema::kompetisi;

    diesel::insert_into(kompetisi::table)
        .values(&kompetisi_data)
        .get_result::<Kompetisi>(connection)
}

pub fn find_many_kompetisi<'a>(
    connection: &UnwrappedPool,
    kompetisi_data: CreateKompetisi,
) -> Result<Kompetisi, diesel::result::Error> {
    use crate::schema::kompetisi;

    diesel::insert_into(kompetisi::table)
        .values(&kompetisi_data)
        .get_result::<Kompetisi>(connection)
}

pub fn propose_kompetisi(
    connection: &UnwrappedPool,
    token: String,
    kompetisi_data: ProposeKompetisiInput,
) -> Result<String, actix_web::error::Error> {
    let claim = match crate::modules::auth::service::get_token_data(token) {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(data) => data,
    };

    match create_kompetisi(
        connection,
        CreateKompetisi {
            user_id: claim.user_id,
            no_telp: kompetisi_data.no_telp,
            nama_lembaga_pendaftar: kompetisi_data.nama_lembaga_pendaftar,
            nama_kompetisi: kompetisi_data.nama_kompetisi,
            kategori_kompetisi: kompetisi_data.kategori_kompetisi,
            deskripsi_kompetisi: kompetisi_data.deskripsi_kompetisi,
            tags_kompetisi: kompetisi_data.tags_kompetisi,
            tanggal_pelaksanaan: kompetisi_data.tanggal_pelaksanaan,
            batas_awal_registrasi: kompetisi_data.batas_awal_registrasi,
            batas_akhir_registrasi: kompetisi_data.batas_akhir_registrasi,
            link_registrasi: kompetisi_data.link_registrasi,
            link_website: kompetisi_data.link_website,
            link_linkedin: kompetisi_data.link_linkedin,
            akun_instagram: kompetisi_data.akun_instagram,
            id_line: kompetisi_data.id_line,
            akun_twitter: kompetisi_data.akun_twitter,
            link_poster: kompetisi_data.link_poster,
            ..CreateKompetisi::default()
        },
    ) {
        Ok(_) => Ok("Success!".to_string()),
        Err(e) => Err(ErrorBadRequest(e)),
    }
}

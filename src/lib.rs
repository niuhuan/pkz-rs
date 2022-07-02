use anyhow::Context;
pub use anyhow::Result;
use async_trait::async_trait;
use async_zip::read::fs::ZipFileReader;
use async_zip::write::{EntryOptions, ZipFileWriter};
use async_zip::Compression;
use itertools::Itertools;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use uuid::Uuid;

pub use entities::*;

#[cfg(test)]
mod tests;

mod entities;

#[async_trait]
pub trait ComicLoader {
    async fn archive_info(&self) -> Result<ArchiveInfo>;
    async fn archive_cover(&self) -> Result<Vec<u8>>;
    async fn archive_author_avatar(&self) -> Result<Vec<u8>>;
    async fn comic_count(&self) -> Result<i64>;
    async fn comic_info(&self, comic_idx: i64) -> Result<ComicInfo>;
    async fn comic_cover(&self, comic_idx: i64, comic_info: &ComicInfo) -> Result<Option<Vec<u8>>>;
    async fn comic_author_avatar(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
    ) -> Result<Option<Vec<u8>>>;
    async fn volume_count(&self, comic_idx: i64, comic_info: &ComicInfo) -> Result<i64>;
    async fn volume_info(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
    ) -> Result<VolumeInfo>;
    async fn volume_cover(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
    ) -> Result<Option<Vec<u8>>>;
    async fn chapter_count(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
    ) -> Result<i64>;
    async fn chapter_info(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
        chapter_idx: i64,
    ) -> Result<ChapterInfo>;
    async fn chapter_cover(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
        chapter_idx: i64,
        chapter_info: &ChapterInfo,
    ) -> Result<Option<Vec<u8>>>;
    async fn picture_count(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
        chapter_idx: i64,
        chapter_info: &ChapterInfo,
    ) -> Result<i64>;
    async fn picture_info(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
        chapter_idx: i64,
        chapter_info: &ChapterInfo,
        picture_idx: i64,
    ) -> Result<PictureInfo>;
    async fn picture_data(
        &self,
        comic_idx: i64,
        comic_info: &ComicInfo,
        volume_idx: i64,
        volume_info: &VolumeInfo,
        chapter_idx: i64,
        chapter_info: &ChapterInfo,
        picture_idx: i64,
        picture_info: &PictureInfo,
    ) -> Result<Vec<u8>>;
}

async fn write_to_pkz<W: AsyncWrite + Unpin>(
    writer: &mut ZipFileWriter<W>,
    path: String,
    data: Vec<u8>,
) -> Result<()> {
    let opts = EntryOptions::new(String::from(path), Compression::Deflate);
    let data: Vec<u8> = data.iter().map(|x| x ^ 170).collect_vec();
    writer.write_entry_whole(opts, &data).await?;
    Ok(())
}

pub async fn write_pkz<W: AsyncWrite + Unpin>(
    writer: W,
    loader: Box<dyn ComicLoader>,
) -> Result<()> {
    let mut writer = ZipFileWriter::new(writer);

    let archive_info = loader.archive_info().await?;

    let mut archive = Archive {
        name: archive_info.name.clone(),
        author: archive_info.author.clone(),
        description: archive_info.description.clone(),
        cover_path: "".to_string(),
        author_avatar_path: "".to_string(),
        comic_count: loader.comic_count().await?,
        volumes_count: 0,
        chapter_count: 0,
        picture_count: 0,
        comics: vec![],
    };

    for comic_idx in 0..archive.comic_count {
        let comic_info = loader.comic_info(comic_idx).await?;

        let mut comic = Comic {
            id: comic_info.id.clone(),
            title: comic_info.title.clone(),
            categories: vec![],
            tags: vec![],
            author_id: comic_info.author_id.clone(),
            author: comic_info.author.clone(),
            updated_at: comic_info.updated_at.clone(),
            created_at: comic_info.created_at.clone(),
            description: comic_info.description.clone(),
            chinese_team: comic_info.chinese_team.clone(),
            finished: comic_info.finished.clone(),
            idx: comic_idx,
            cover_path: "".to_string(),
            author_avatar_path: "".to_string(),
            volumes_count: loader.volume_count(comic_idx, &comic_info).await?,
            chapter_count: 0,
            picture_count: 0,
            volumes: vec![],
        };

        archive.volumes_count += comic.volumes_count;

        if let Some(comic_cover_data) = loader.comic_cover(comic_idx, &comic_info).await? {
            let path = Uuid::new_v4().to_string();
            write_to_pkz(&mut writer, path.clone(), comic_cover_data).await?;
            comic.cover_path = path
        }

        if let Some(comic_cover_data) = loader.comic_author_avatar(comic_idx, &comic_info).await? {
            let path = Uuid::new_v4().to_string();
            write_to_pkz(&mut writer, path.clone(), comic_cover_data).await?;
            comic.author_avatar_path = path
        }

        for volume_idx in 0..comic.volumes_count {
            let volume_info = loader
                .volume_info(comic_idx, &comic_info, volume_idx)
                .await?;

            let mut volume = Volume {
                id: volume_info.id.clone(),
                title: volume_info.title.clone(),
                updated_at: volume_info.updated_at.clone(),
                created_at: volume_info.created_at.clone(),
                idx: volume_idx,
                cover_path: "".to_string(),
                chapter_count: loader
                    .chapter_count(comic_idx, &comic_info, volume_idx, &volume_info)
                    .await?,
                picture_count: 0,
                chapters: vec![],
            };

            comic.chapter_count += volume.chapter_count;
            archive.chapter_count += volume.chapter_count;

            if let Some(cover_data) = loader
                .volume_cover(comic_idx, &comic_info, volume_idx, &volume_info)
                .await?
            {
                let path = Uuid::new_v4().to_string();
                write_to_pkz(&mut writer, path.clone(), cover_data).await?;
                volume.cover_path = path
            }

            for chapter_idx in 0..volume.chapter_count {
                let chapter_info = loader
                    .chapter_info(
                        comic_idx,
                        &comic_info,
                        volume_idx,
                        &volume_info,
                        chapter_idx,
                    )
                    .await?;

                let mut chapter = Chapter {
                    id: chapter_info.id.clone(),
                    title: chapter_info.title.clone(),
                    updated_at: chapter_info.updated_at.clone(),
                    created_at: chapter_info.created_at.clone(),
                    idx: chapter_idx,
                    cover_path: "".to_string(),
                    picture_count: loader
                        .picture_count(
                            comic_idx,
                            &comic_info,
                            volume_idx,
                            &volume_info,
                            chapter_idx,
                            &chapter_info,
                        )
                        .await?,
                    pictures: vec![],
                };

                volume.chapter_count += chapter.picture_count;
                comic.chapter_count += chapter.picture_count;
                archive.chapter_count += chapter.picture_count;

                if let Some(cover_data) = loader
                    .chapter_cover(
                        comic_idx,
                        &comic_info,
                        volume_idx,
                        &volume_info,
                        chapter_idx,
                        &chapter_info,
                    )
                    .await?
                {
                    let path = Uuid::new_v4().to_string();
                    write_to_pkz(&mut writer, path.clone(), cover_data).await?;
                    chapter.cover_path = path
                }

                for picture_idx in 0..chapter.picture_count {
                    let picture_info = loader
                        .picture_info(
                            comic_idx,
                            &comic_info,
                            volume_idx,
                            &volume_info,
                            chapter_idx,
                            &chapter_info,
                            picture_idx,
                        )
                        .await?;
                    let mut picture = Picture {
                        id: picture_info.id.clone(),
                        title: picture_info.title.clone(),
                        width: picture_info.width.clone(),
                        height: picture_info.height.clone(),
                        format: picture_info.format.clone(),
                        idx: picture_idx,
                        picture_path: "".to_string(),
                    };
                    let picture_data = loader
                        .picture_data(
                            comic_idx,
                            &comic_info,
                            volume_idx,
                            &volume_info,
                            chapter_idx,
                            &chapter_info,
                            picture_idx,
                            &picture_info,
                        )
                        .await?;

                    let path = Uuid::new_v4().to_string();
                    write_to_pkz(&mut writer, path.clone(), picture_data).await?;
                    picture.picture_path = path;

                    chapter.pictures.push(picture);
                }
                volume.chapters.push(chapter);
            }
            comic.volumes.push(volume);
        }
        archive.comics.push(comic);
    }

    let inf = serde_json::to_string(&archive)?;

    write_to_pkz(&mut writer, "PKZ-INFO".to_owned(), inf.into_bytes()).await?;

    writer.close().await?;

    Ok(())
}

pub async fn read_pkz_file(pkz_path: String, inner_path: String) -> Result<Vec<u8>> {
    let zip = ZipFileReader::new(pkz_path).await?;
    let (idx, _) = zip.entry(&inner_path).with_context(|| "not found")?;
    let mut reader = zip.entry_reader(idx).await?;
    let mut buff: Vec<u8> = vec![];
    reader.read_to_end(&mut buff).await?;
    Ok(buff)
}

pub async fn read_pkz(pkz_path: String) -> Result<Archive> {
    Ok(serde_json::from_str(&String::from_utf8(
        read_pkz_file(pkz_path, "PKZ-INFO".to_owned()).await?,
    )?)?)
}

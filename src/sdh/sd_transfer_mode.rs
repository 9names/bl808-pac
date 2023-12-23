#[doc = "Register `sd_transfer_mode` reader"]
pub struct R(crate::R<SD_TRANSFER_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_TRANSFER_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_TRANSFER_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_TRANSFER_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_transfer_mode` writer"]
pub struct W(crate::W<SD_TRANSFER_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_TRANSFER_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_TRANSFER_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_TRANSFER_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_en` reader - "]
pub type DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `dma_en` writer - "]
pub type DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_TRANSFER_MODE_SPEC, bool, O>;
#[doc = "Field `blk_cnt_en` reader - "]
pub type BLK_CNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `blk_cnt_en` writer - "]
pub type BLK_CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_TRANSFER_MODE_SPEC, bool, O>;
#[doc = "Field `auto_cmd_en` reader - "]
pub type AUTO_CMD_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `auto_cmd_en` writer - "]
pub type AUTO_CMD_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_TRANSFER_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `to_host_dir` reader - "]
pub type TO_HOST_DIR_R = crate::BitReader<bool>;
#[doc = "Field `to_host_dir` writer - "]
pub type TO_HOST_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_TRANSFER_MODE_SPEC, bool, O>;
#[doc = "Field `multi_blk_sel` reader - "]
pub type MULTI_BLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `multi_blk_sel` writer - "]
pub type MULTI_BLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_TRANSFER_MODE_SPEC, bool, O>;
#[doc = "Field `reserved_15_6` reader - "]
pub type RESERVED_15_6_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn blk_cnt_en(&self) -> BLK_CNT_EN_R {
        BLK_CNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn auto_cmd_en(&self) -> AUTO_CMD_EN_R {
        AUTO_CMD_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn to_host_dir(&self) -> TO_HOST_DIR_R {
        TO_HOST_DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn multi_blk_sel(&self) -> MULTI_BLK_SEL_R {
        MULTI_BLK_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn reserved_15_6(&self) -> RESERVED_15_6_R {
        RESERVED_15_6_R::new((self.bits >> 6) & 0x03ff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<0> {
        DMA_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn blk_cnt_en(&mut self) -> BLK_CNT_EN_W<1> {
        BLK_CNT_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_en(&mut self) -> AUTO_CMD_EN_W<2> {
        AUTO_CMD_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn to_host_dir(&mut self) -> TO_HOST_DIR_W<4> {
        TO_HOST_DIR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn multi_blk_sel(&mut self) -> MULTI_BLK_SEL_W<5> {
        MULTI_BLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_transfer_mode](index.html) module"]
pub struct SD_TRANSFER_MODE_SPEC;
impl crate::RegisterSpec for SD_TRANSFER_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_transfer_mode::R](R) reader structure"]
impl crate::Readable for SD_TRANSFER_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_transfer_mode::W](W) writer structure"]
impl crate::Writable for SD_TRANSFER_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_transfer_mode to value 0"]
impl crate::Resettable for SD_TRANSFER_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

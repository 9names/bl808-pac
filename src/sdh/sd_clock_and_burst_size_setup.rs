#[doc = "Register `sd_clock_and_burst_size_setup` reader"]
pub struct R(crate::R<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_clock_and_burst_size_setup` writer"]
pub struct W(crate::W<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>;
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
impl From<crate::W<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `brst_size` reader - "]
pub type BRST_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `brst_size` writer - "]
pub type BRST_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `dma_size` reader - "]
pub type DMA_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dma_size` writer - "]
pub type DMA_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `priority` reader - "]
pub type PRIORITY_R = crate::BitReader<bool>;
#[doc = "Field `priority` writer - "]
pub type PRIORITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, bool, O>;
#[doc = "Field `axi_non_post_wr` reader - "]
pub type AXI_NON_POST_WR_R = crate::BitReader<bool>;
#[doc = "Field `axi_non_post_wr` writer - "]
pub type AXI_NON_POST_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, bool, O>;
#[doc = "Field `rd_endian` reader - "]
pub type RD_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `rd_endian` writer - "]
pub type RD_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, bool, O>;
#[doc = "Field `wr_endian` reader - "]
pub type WR_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `wr_endian` writer - "]
pub type WR_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, bool, O>;
#[doc = "Field `reserved_13_8` reader - "]
pub type RESERVED_13_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rd_ostdg` reader - "]
pub type RD_OSTDG_R = crate::BitReader<bool>;
#[doc = "Field `rd_ostdg` writer - "]
pub type RD_OSTDG_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, bool, O>;
#[doc = "Field `wr_ostdg` reader - "]
pub type WR_OSTDG_R = crate::BitReader<bool>;
#[doc = "Field `wr_ostdg` writer - "]
pub type WR_OSTDG_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn brst_size(&self) -> BRST_SIZE_R {
        BRST_SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dma_size(&self) -> DMA_SIZE_R {
        DMA_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn axi_non_post_wr(&self) -> AXI_NON_POST_WR_R {
        AXI_NON_POST_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_endian(&self) -> RD_ENDIAN_R {
        RD_ENDIAN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wr_endian(&self) -> WR_ENDIAN_R {
        WR_ENDIAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn reserved_13_8(&self) -> RESERVED_13_8_R {
        RESERVED_13_8_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rd_ostdg(&self) -> RD_OSTDG_R {
        RD_OSTDG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_ostdg(&self) -> WR_OSTDG_R {
        WR_OSTDG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn brst_size(&mut self) -> BRST_SIZE_W<0> {
        BRST_SIZE_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_size(&mut self) -> DMA_SIZE_W<2> {
        DMA_SIZE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<4> {
        PRIORITY_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn axi_non_post_wr(&mut self) -> AXI_NON_POST_WR_W<5> {
        AXI_NON_POST_WR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rd_endian(&mut self) -> RD_ENDIAN_W<6> {
        RD_ENDIAN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn wr_endian(&mut self) -> WR_ENDIAN_W<7> {
        WR_ENDIAN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn rd_ostdg(&mut self) -> RD_OSTDG_W<14> {
        RD_OSTDG_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn wr_ostdg(&mut self) -> WR_OSTDG_W<15> {
        WR_OSTDG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock and Burst Size Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_clock_and_burst_size_setup](index.html) module"]
pub struct SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC;
impl crate::RegisterSpec for SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_clock_and_burst_size_setup::R](R) reader structure"]
impl crate::Readable for SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_clock_and_burst_size_setup::W](W) writer structure"]
impl crate::Writable for SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_clock_and_burst_size_setup to value 0xce"]
impl crate::Resettable for SD_CLOCK_AND_BURST_SIZE_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0xce;
}
